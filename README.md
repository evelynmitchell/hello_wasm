# Rust WASM Hello World

A simple "Hello World" project demonstrating how to compile Rust to WebAssembly and run it in the browser.

## What This Project Does

This project exports two Rust functions to JavaScript via WebAssembly:
- `greet(name)`: Returns a personalized greeting message
- `add(a, b)`: Adds two numbers together

The web interface allows you to interact with these WASM functions directly in your browser.

## Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/)
- **wasm-pack**: Tool for building and packaging Rust WASM projects
- **Python 3**: For running a local web server (or any other HTTP server)

## How This Project Was Built

### 1. Install Rust Toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

### 2. Install wasm-pack

```bash
cargo install wasm-pack
```

### 3. Create a New Rust Library

```bash
cargo new --lib hello-wasm
cd hello-wasm
```

### 4. Configure Cargo.toml

Edit `Cargo.toml` to set up WASM compilation:

```toml
[package]
name = "hello-wasm"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

**Key points:**
- `crate-type = ["cdylib"]`: Creates a dynamic library compatible with WASM
- `wasm-bindgen`: Enables JavaScript interop

### 5. Write Rust Functions

In `src/lib.rs`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

The `#[wasm_bindgen]` attribute exports functions to JavaScript.

### 6. Build the WASM Module

```bash
wasm-pack build --target web
```

This generates:
- `pkg/hello_wasm_bg.wasm`: The compiled WebAssembly binary
- `pkg/hello_wasm.js`: JavaScript bindings for loading and calling WASM functions
- `pkg/hello_wasm.d.ts`: TypeScript type definitions

### 7. Create the HTML Interface

Create `index.html` that imports and uses the WASM module:

```html
<script type="module">
    import init, { greet, add } from './pkg/hello_wasm.js';

    async function run() {
        await init();
        // WASM functions are now available
    }

    run();
</script>
```

## How to Run

### 1. Build the WASM Module

```bash
wasm-pack build --target web
```

### 2. Start a Local Web Server

```bash
python3 -m http.server 8000
```

Or use any other HTTP server:
```bash
# Using Node.js
npx http-server

# Using Rust
cargo install simple-http-server
simple-http-server
```

### 3. Open in Browser

Navigate to `http://localhost:8000` in your web browser.

**Note:** A web server is required because browsers block ES6 module imports from `file://` URLs due to CORS restrictions.

## Project Structure

```
/
├── .gitignore          # Excludes build artifacts
├── Cargo.toml          # Rust project configuration
├── Cargo.lock          # Dependency lock file
├── README.md           # This file
├── Claude.md           # Claude Code guidance
├── LICENSE             # Project license
├── index.html          # Web interface
├── src/
│   └── lib.rs          # Rust source code
├── pkg/                # Generated WASM and JS bindings (not in git)
│   ├── hello_wasm.js
│   ├── hello_wasm_bg.wasm
│   └── ...
├── target/             # Build artifacts (not in git)
└── Summary/            # Daily work summaries
    └── summary20251027.md
```

## Testing

This project includes comprehensive unit tests for both functions.

### Run Tests

```bash
cargo test
```

### Test Coverage

**greet() function tests:**
- Normal names
- Empty strings
- Single characters
- Long names
- Special characters (Unicode)
- Numbers in names
- Names with spaces

**add() function tests:**
- Combinatorial testing: +/+, +/-, -/-, -/+
- Zero edge cases
- Boundary values
- Large numbers near i32 limits

All tests follow Rust's snake_case naming convention and test both typical usage and edge cases.

### Test Output

```
running 14 tests
test tests::test_add_boundary_values ... ok
test tests::test_add_max_values ... ok
test tests::test_add_negative_negative ... ok
test tests::test_add_negative_positive ... ok
test tests::test_add_positive_negative ... ok
test tests::test_add_positive_positive ... ok
test tests::test_add_with_zero ... ok
test tests::test_greet_empty_string ... ok
test tests::test_greet_long_name ... ok
test tests::test_greet_normal_name ... ok
test tests::test_greet_numbers ... ok
test tests::test_greet_single_char ... ok
test tests::test_greet_spaces ... ok
test tests::test_greet_special_chars ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Making Changes

### Modify Rust Code

1. Edit `src/lib.rs`
2. Run tests to verify changes:
   ```bash
   cargo test
   ```
3. Rebuild the WASM module:
   ```bash
   wasm-pack build --target web
   ```
4. Refresh your browser (hard refresh may be needed: Ctrl+Shift+R or Cmd+Shift+R)

### Modify HTML/CSS

1. Edit `index.html`
2. Refresh your browser

## How It Works

### The Build Process

1. **Rust → WASM**: `rustc` compiles Rust code to WebAssembly bytecode
2. **wasm-bindgen**: Generates JavaScript "glue code" that:
   - Loads the WASM module
   - Converts between JavaScript and Rust types
   - Exposes Rust functions to JavaScript
3. **wasm-opt**: Optimizes the WASM binary for size and performance

### Runtime Execution

1. Browser loads `index.html`
2. JavaScript imports the WASM module
3. `init()` function loads and instantiates the WASM binary
4. Rust functions become available as JavaScript functions
5. When called, JavaScript values are converted to Rust types, function executes in WASM, result is converted back to JavaScript

## Debugging

### Check Browser Console

Open Developer Tools (F12) and check the Console tab for:
- WASM loading errors
- JavaScript errors
- `console.log()` output

### Common Issues

**WASM module fails to load:**
- Ensure you're using a web server (not `file://`)
- Check that `pkg/` directory exists and contains WASM files
- Try a hard refresh (Ctrl+Shift+R)

**Functions return unexpected values:**
- Check type conversions between JavaScript and Rust
- Verify function signatures match between Rust and JavaScript

**Build errors:**
- Ensure Rust and wasm-pack are installed correctly
- Run `cargo clean` and rebuild

## Next Steps

Potential enhancements:
- Add more complex functions (array processing, async operations)
- Integrate with a JavaScript framework (React, Vue, Svelte)
- Add Rust unit tests
- Optimize WASM bundle size
- Use web workers for heavy computations
- Implement memory sharing between JS and WASM

## Resources

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [wasm-pack Documentation](https://rustwasm.github.io/docs/wasm-pack/)
- [MDN WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)

## License

See LICENSE file in repository root.
