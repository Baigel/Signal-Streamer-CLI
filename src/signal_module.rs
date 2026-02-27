/* 
 * Author: Ahmed Baig
 * Description: Takes a csv file and streams the values from the file at a set frequency across a network port
 * Date: September 2025
 * AI Disclosure: No AI generated code present
*/

use csv;
use std::net::UdpSocket;
use rand::prelude::*;
use simper_filter::Svf;
use simper_filter::FilterType;

pub struct Signal {
    /* Private Members */
    // Streaming vars
    socket: UdpSocket,
    socket_addr_str: String,
    // Signal vars
    signal_data: Vec<f32>,
    signal_index: usize,
    ms_delay: i32,
    skip_n: usize,
    int_mode: bool,
    // Filter vars
    lp_filter: Svf<f32>,
    hp_filter: Svf<f32>,
    // Noise vars
    rand_rng: ThreadRng,
    noise_level: i32,
}

impl Signal {

    /* Public Function Definitions */

    // Associated Functions

    pub fn new() -> Signal {
        Signal { // default setup
            // Streaming vars
            socket: UdpSocket::bind("0.0.0.0:0").unwrap(), // Bind to don't care addr
            socket_addr_str: String::new(),
            // Signal vars
            signal_data: Vec::new(),
            signal_index: 0,
            ms_delay: 5, // Default of 200 samples/s
            skip_n: 0,
            int_mode: true,
            // Filters
            lp_filter: Svf::new(FilterType::Lowpass, 1000.0, 500.0, 0.771, 0.0).unwrap(),
            hp_filter: Svf::new(FilterType::Lowpass, 1000.0, 0.0, 0.771, 0.0).unwrap(),
            // Noise vars
            noise_level: 0,
            rand_rng: rand::rng(),
        }
    }

    // Methods

    pub fn read_csv_file(&mut self, file_name: String) { // TODO: need to add support for more separators
        let mut rdr = csv::Reader::from_path(file_name).expect("ERR: Expected csv");
        for result in rdr.records() {
            let record: csv::StringRecord = result.expect("ERR: Expected csv");
            let string_vector: Vec<String> = record.iter().map(String::from).collect(); // Collect each row into a single string
            self.signal_data.push(string_vector[0].parse::<f32>().expect("ERR: File contains non-int type"));
        }

        // TODO: should be able to turn this autonormalizing off - that will break noise functionality and potentially some other stuff
        // Transform vector to range: -1 -> 1
        let max: f32 = self.signal_data.iter().copied().reduce(f32::max).expect("ERR: signal vector contained unexpected value");
        let min: f32 = self.signal_data.iter().copied().reduce(f32::min).expect("ERR: signal vector contained unexpected value");
        self.signal_data = self.signal_data.iter().map(|x| (x - min) * 2.0 / (max - min) - 1.0).collect();

    }

    pub fn set_network_socket(&mut self, port: String) {
        self.socket_addr_str.push_str(format!("127.0.0.1:{}", port).as_str());
        println!("Using Network Socket: {}", self.socket_addr_str);
    }

    pub fn set_lp_filter(&mut self, sample_f: i32, cutoff: i32) {
        println!("LP Filter cutoff:     {}", cutoff);
        let _ = self.lp_filter.set(FilterType::Lowpass, sample_f as f32, cutoff as f32, 0.771, 0.0);
    }
    
    pub fn set_hp_filter(&mut self, sample_f: i32, cutoff: i32) {
        println!("HP Filter cutoff:     {}", cutoff);
        let _ = self.hp_filter.set(FilterType::Highpass, sample_f as f32, cutoff as f32, 0.771, 0.0);
    }
    
    pub fn set_noise(&mut self, noise_level: i32) {
        self.noise_level = noise_level;
        println!("Noise Level:          {}", self.noise_level);
    }

    pub fn set_sample_f(&mut self, sample_f: i32) {
        self.ms_delay = (1000 / sample_f) as i32;
        println!("Sample Freq:          {}", sample_f);
    }
    
    pub fn set_skip_n(&mut self, n: usize) {
        self.skip_n = n;
        println!("Skipping n samples:   {}", self.skip_n);
    }

    pub fn set_int_mode(&mut self, int_mode: bool) {
        self.int_mode = int_mode;
        if int_mode {
            println!("Mode:                 ints (0 -> 1000)");
        } else {
            println!("Mode:                 floats (-1 -> 1)");
        }
    }
 
    pub fn get_ms_delay(&self) -> i32 {
        self.ms_delay
    }

    pub fn transmit_data(&mut self) {
        let next_data: f32 = self.get_next_data();
        self.socket.send_to(&format!("{}", next_data).into_bytes(), self.socket_addr_str.clone()).expect("ERR: Invalid port");
    }

    /* Private Function Definitions */

    fn get_next_data(&mut self) -> f32 {
        // Get Next data point
        let mut data: f32 = self.signal_data[self.signal_index % self.signal_data.len()];
        self.signal_index += 1 + self.skip_n; // Skip n samples every transmission

        // Add noise
        data = data + (self.rand_rng.random::<f32>() * 2.0 - 1.0) * (self.noise_level as f32) / 100.0;

        // Apple Filters
        data = self.lp_filter.tick(data);
        data = self.hp_filter.tick(data);

        // Order of operations note: converting to the range of 0 -> 1000 must be done post noise/filtering/etc as that code only works when the range is floats between -1 -> 1.

        // Convert to int 0 -> 1000 // it seems that the filtering/noise process pushes the limits of the signal beyond 0 and 1000, and -1 to 1 when those are the ranges
        if self.int_mode {
            data += 1 as f32; // -1 -> 1 to 0 -> 2
            data *= 500 as f32; // 0 -> 2 to 0 -> 1000
            data = data.round();
        }

        data
    }

}

