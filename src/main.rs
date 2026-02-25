/*
 * Author: Ahmed Baig
 * Description: Takes a csv file and streams the values from the file at a set frequency across a network port
 * Date: September 2025
 * AI Disclosure: No AI generated code present
*/

/* Library Includes */

use String;
use clap::Parser; // CLI tool
use tokio::time::{interval, Duration};

/* Modules */
pub mod signal_module;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// CSV File (single column only)
    #[arg(short, long)]
    filename: String,

    /// Network Port (127.0.0.1:xxxxx)
    #[arg(short, long, default_value_t = String::from("12345"))]
    port: String,

    /// Sample freq of data provided
    #[arg(long, default_value_t = 200)]
    sample_f: i32,

    /// Skip n samples every transmission
    #[arg(long, default_value_t = 0)]
    skip_n: usize,

    /// LP Filter (default: same as sample_f)
    #[arg(long, default_value_t = 200)]
    lp_filter: i32,

    /// HP Filter
    #[arg(long, default_value_t = 0)]
    hp_filter: i32,

    /// Noise Level Percent (0 -> 100)
    #[arg(long, default_value_t = 0)]
    noise: i32,

     /// Default is floats from -1 -> +1 (int mode: ints from 0 -> 1000)
    #[arg(long, default_value_t = false)]
    int_mode: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {

    let args = Args::parse();

    let mut signal: signal_module::Signal = signal_module::Signal::new();
    signal.set_network_socket(args.port);
    signal.set_lp_filter(args.sample_f, args.lp_filter);
    signal.set_hp_filter(args.sample_f, args.hp_filter);
    signal.set_noise(args.noise);
    signal.set_sample_f(args.sample_f);
    signal.set_skip_n(args.skip_n);
    signal.set_int_mode(args.int_mode);
    signal.read_csv_file(args.filename);

    println!("Starting transmission now...");

    let mut interval = interval(Duration::from_millis(signal.get_ms_delay() as u64));
    loop {
        signal.transmit_data();
        interval.tick().await;
    }
}
