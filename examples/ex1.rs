use std::env;
use std::fs::File;
use std::io::BufReader;

extern crate toy_png_decoder_rs;

fn main() {
    let filename = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };
    let f = BufReader::new(File::open(filename).expect("file not found"));
    
    toy_png_decoder_rs::show_chunks_info(f);
}