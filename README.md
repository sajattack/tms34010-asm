# tms34010-asm

## Installation
- Follow instructions to install rust at https://rustup.rs/
- then run the following
 ```sh
 cargo install --git https://github.com/sajattack/tms34010-asm
 ```
 
 ## Building from a git checkout (so you can make changes)
 ```sh
git clone https://github.com/sajattack/tms34010-asm.git
cd tms34010-asm
cargo build --release
```
output binary will be under `/target` after running the above steps
or you can build and run in one operation by replacing 

`cargo build --release` 

with 

`cargo run --release -- <args>`

## Disassembler usage
```
TMS34010 Disassembler 0.1.0
Paul Sajna, hello@paulsajna.com
Disassembler for Texas Instruments TMS34010 CPU

USAGE:
    tms34010-disasm [OPTIONS] <in_file>

ARGS:
    <in_file>    File to disassemble

OPTIONS:
    -h, --help               Print help information
    -o, --offset <offset>    Seek N bytes in in_file before starting disassembly [default: 0]
    -p, --pc <start_pc>      Initial program counter at start of file or seek address [default: 0]
    -s, --size <size>        Limit number of bytes to disassemble
    -V, --version            Print version information
```
