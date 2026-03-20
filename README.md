# Signal Streamer CLI

## What is this?
This little tool allows you to stream any csv file containing a list of numbers across a network port. It has adjustable parameters that you can adjust too.

## Who is this for?
The Signal Streamer CLI provides a convenient way of generating a real time signal. Anyone who works on analysing realtime signals can use the provided example code to connect their project to a continuous stream of incoming signal, without requiring physical hardware.

## Features
 - Normalizing: normalizes signal to -1 -> 1 or 0 -> 1000 when in integer only mode
 - Port: the CLI transmits to any local port and defaults to 127.0.0.1:12345
 - Sample rate: replay the provided data at any sample rate up to 1000 Hz
 - Skip samples: only transmit every nth sample
 - Filter: adjustable cutoff high pass and low pass filters
 - Noise: add noise up to 100% of level of signal

## Build
Signal Streamer CLI:\
`cargo build --bin Signal-Streamer-CLI`

Receiver Demo:\
`cargo build --bin receive-demo`

## Usage
```
Usage: Signal-Streamer-CLI [OPTIONS] --filename <FILENAME>

Options:
Usage: Signal-Streamer-CLI [OPTIONS] --filename <FILENAME>

Options:
  -f, --filename <FILENAME>    CSV File (single column only)
  -p, --port <PORT>            			Network Port (127.0.0.1:xxxxx) [default: 12345]
      --sample-f <SAMPLE_F>    	Sample freq of data provided [default: 200]
      --skip-n <SKIP_N>        		Skip n samples every transmission [default: 0]
      --lp-filter <LP_FILTER>  		LP Filter (default: same as sample_f) [default: 200]
      --hp-filter <HP_FILTER>  		HP Filter [default: 0]
      --normalize              				Normalize (from -1 to 1, or 0 to 1000 in int mode)
      --noise <NOISE>          			Noise Level Percent (0 -> 100) [default: 0]
      --int-mode               				Default is floats from -1 -> +1 (int mode: ints from 0 -> 1000)
  -h, --help                   					Print help
  -V, --version                				Print version
```

## Example
Run the following commands in two separate terminals: (two demo scripts are provided for receiving, one in rust, the other in python)
`cargo run --bin Signal-Streamer-CLI -- -f ./Test-Signal.csv --int-mode`\
`cargo run --bin receive-demo // rust option`
`python3 ./Python_Receive.py // python option`
