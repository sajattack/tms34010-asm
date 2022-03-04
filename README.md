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

`cargo run --release`
