extern crate env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use colored::Colorize;
use humantime::format_duration;
use std::collections;
use std::collections::VecDeque;
use std::error;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path;
use std::time;

#[derive(Parser, Debug)]
#[clap(author = "Jason Reilly", version = "0.0.1", about = "windowed search", long_about = None)]
struct Args {
    #[clap(short = 'i', long = "input", required = true)]
    input: String,

    #[clap(short = 'f', long = "file", required = true, parse(from_os_str))]
    file: path::PathBuf,

    #[clap(short = 's', long = "size (even number)", default_value_t = 10)]
    size: usize,
}
fn main() -> Result<(), Box<dyn error::Error>> {
    let start = time::Instant::now();
    env_logger::init();

    let args = Args::parse();
    debug!("{:?}", args);

    let search = &args.input;
    let path = &args.file;
    let size = *&args.size as usize;

    let br = io::BufReader::new(fs::File::open(path.as_path()).unwrap());

    let mut count = 1;
    let mut cache: collections::VecDeque<String> = VecDeque::new();
    let end_of_starting_range = (size / 2) - 2; // since a vec indexes at 0
    for line in br.lines() {
        let line = line.unwrap();
        // debug!("length of cache: {}", cache.len());

        if cache.len() == size {
            cache.pop_front();
            let center_string = cache.get(end_of_starting_range + 1).unwrap();
            if center_string.contains(search) {
                println!("{}", format!("------------ {} ------------", count).bold().green());
                let start_range = 0..=end_of_starting_range;
                debug!("start_range: {:?}", start_range);
                start_range.for_each(|a| println!("{}", cache.get(a).unwrap()));
                println!("{}", format!("{}", cache.get(end_of_starting_range + 1).unwrap()).bold().red());
                let end_range = (end_of_starting_range + 2)..(size - 1);
                debug!("end range: {:?}", end_range);
                end_range.for_each(|a| println!("{}", cache.get(a).unwrap()));
                count = count + 1;
            }
        }
        cache.push_back(line);
    }
    info!("Duration: {}", format_duration(start.elapsed()).to_string());
    Ok(())
}
