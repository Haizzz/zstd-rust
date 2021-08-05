extern crate clap;
use clap::ArgMatches;
use std::fs;

use crate::utils::{bit_at_index, exit_with_message};

mod utils;

// 0xFD2FB528 as u8
const MAGIC_NUMBER: [u8; 4] = [40, 181, 47, 253];

fn read_or_error(path: &str) -> Vec<u8> {
    // given a file path, read the content with error handling
    let file_content = match fs::read(path) {
        Ok(content) => content,
        Err(e) => {
            let err_msg = "Error reading file ".to_owned() + path + ":" + e.to_string().as_str();
            exit_with_message(err_msg.as_str())
        }
    };
    file_content
}

fn compress(path: &str) {}

fn decompress(path: &str) {
    let content = read_or_error(path);

    // parse the content of the file
    let magic_number = &content[0..4];
    for i in 0..4 {
        if MAGIC_NUMBER[i] != magic_number[i] {
            exit_with_message("Corrupted file: magic number does not match");
        }
    }

    // parse frame header
    let frame_header_descriptor = content[4];
    let frame_content_size_flag = bit_at_index(frame_header_descriptor, 6);
    let fcs_field_size = bit_at_index(frame_header_descriptor, 7);
    let single_segment_flag = bit_at_index(frame_header_descriptor, 5);
    let content_checksum_flag = bit_at_index(frame_header_descriptor, 2);
    let dictionary_id_flag = bit_at_index(frame_header_descriptor, 0);
    let did_field_size = bit_at_index(frame_header_descriptor, 0);
}

pub fn run(args: ArgMatches) -> () {
    let decompress_mode = args.is_present("decompress");
    // clap handles required arg so unwrap here is safe
    let file_path = args.value_of("file").unwrap();

    println!("args are: ");
    println!("{}", decompress_mode);
    println!("{}", file_path);
    println!("==========");

    if decompress_mode {
        decompress(file_path)
    } else {
        compress(file_path)
    }
}
