use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

use clap::{Arg, Command};
use tms34010_asm::disasm::disassemble;

fn main() {
    let matches = Command::new("TMS34010 Disassembler")
        .author("Paul Sajna, hello@paulsajna.com")
        .version("0.1.0")
        .about("Disassembler for Texas Instruments TMS34010 CPU")
        .arg(
            Arg::new("in_file")
                .help("File to disassemble")
                .required(true),
        )
        .arg(
            Arg::new("offset")
                .help("Seek N bytes in in_file before starting disassembly")
                .takes_value(true)
                .default_value("0")
                .short('o')
                .long("offset"),
        )
        .arg(
            Arg::new("start_pc")
                .help("Initial program counter at start of file or seek address")
                .takes_value(true)
                .default_value("0")
                .short('p')
                .long("pc"),
        )
        .arg(
            Arg::new("size")
                .help("Limit number of bytes to disassemble")
                .takes_value(true)
                .short('s')
                .long("size"),
        )
        .get_matches();

    let in_file = Path::new(matches.value_of("in_file").unwrap());

    let offset: u64;
    let str_offset = matches.value_of("offset").unwrap();
    if str_offset.starts_with("0x") {
        offset = u64::from_str_radix(str_offset.strip_prefix("0x").unwrap(), 16)
            .expect("Offset is not a valid hexadecimal number");
    } else {
        offset = str_offset
            .parse::<u64>()
            .expect("Offset is not a valid number");
    }

    let start_pc: usize;
    let str_start_pc = matches.value_of("start_pc").unwrap();
    if str_start_pc.starts_with("0x") {
        start_pc = usize::from_str_radix(str_start_pc.strip_prefix("0x").unwrap(), 16)
            .expect("start_pc is not a valid hexadecimal number");
    } else {
        start_pc = str_start_pc
            .parse::<usize>()
            .expect("start_pc is not a valid number");
    }

    let mut size: u64 = 0;
    if let Some(str_size) = matches.value_of("size") {
        if str_size.starts_with("0x") {
            size = u64::from_str_radix(str_size.strip_prefix("0x").unwrap(), 16)
                .expect("size is not a valid hexadecimal number");
        } else {
            size = str_size
                .parse::<u64>()
                .expect("size is not a valid number");
        }
    }

    let mut file = File::open(in_file).expect("Unable to open input file");
    let buffer_size = if size > 0 {
        size
    } else {
        file.metadata().unwrap().len()
    };

    let mut buffer = vec![0u8; buffer_size as usize];
    file.seek(SeekFrom::Start(offset))
        .expect("Unable to seek to offset");
    file.read(buffer.as_mut_slice())
        .expect("Reading from file failed");

    disassemble(buffer.as_mut_slice(), start_pc);
}
