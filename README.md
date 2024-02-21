# Active
<span style="color:red;">Warning:</span>This tool was developed for educational purposes only.

## Instruction
[Install Rust](https://www.rust-lang.org/tools/install)
Clone repo and run 
```
cargo build
```    
## Usage
```
 Usage: cargo run --release -- [OPTIONS] [HOST] [PORT]
```
#### Example
```
cargo run --release -- -t 127.0.0.1 -p 80

TCP Port 80 is closed
```

#### More information
```
cargo run --release -- --help

Usage: cargo run --release -- [OPTIONS] [HOST] [PORT]

        Options:

        -p                      Range of ports to scan

        -u                      UDP scan

        -t                      TCP scan

        --help                  Prints help information

```