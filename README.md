# Groth16 Verifier (WASM)

This project contains a simple Rust program that acts as a Groth16 verifier (using BLS12-381) for a hardcoded proof, compiled to WebAssembly (WASM) for execution with the WebAssembly Micro Runtime (WAMR).

## How to Run

### Prerequisites

1.  **Rust and `wasm32-wasip1` target:**
    Ensure you have Rust installed. If not, follow instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).
    Add the WASI target:

    ```bash
    rustup target add wasm32-wasip1
    ```

2.  **WAMR `iwasm` Runtime:**
    The `iwasm` executable is required to run the WASM module. Build it from source:
    ```bash
    git clone https://github.com/bytecodealliance/wasm-micro-runtime.git
    cd wasm-micro-runtime/product-mini/platforms/darwin # Or your specific OS (linux, windows, etc.)
    mkdir build && cd build
    cmake ..
    make
    # The 'iwasm' executable will be in the current directory.
    ```

### Build the Rust Program

Navigate to the `wasm-verifier` directory and build the Rust program to WASM:

```bash
cd wasm-verifier
cargo build --target wasm32-wasip1 --release
```

This will generate the WASM module at `wasm-verifier/target/wasm32-wasip1/release/wasm-verifier.wasm`.

### Run the WASM Program

Execute the compiled WASM module using the `iwasm` runtime. Make sure you are in the project root directory (`riscv-wasm-zkp-comparison/wasm-verifier`).

```bash
time ./wasm-micro-runtime/product-mini/platforms/darwin/build/iwasm wasm-verifier/target/wasm32-wasip1/release/wasm-verifier.wasm
```

### Expected Output

Upon successful execution, you should see output similar to this:

```
Proof verified successfully!
Verification time: XXX.XXms

real	0mX.XXXs
user	0mX.XXXs
sys	0mX.XXXs
```

The `Verification time` indicates the time taken by the Groth16 verification process within the WASM module. The `real`, `user`, and `sys` times are reported by the `time` command, showing the overall execution time including WASM runtime startup.
