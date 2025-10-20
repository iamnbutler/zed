# GPUI Web Platform

A web-compatible implementation of GPUI that runs in WebAssembly without native OS dependencies.

## Overview

`gpui_web` is a clean-slate implementation of GPUI's core functionality designed specifically for web browsers. Unlike the main `gpui` crate, this implementation:

- Has zero native dependencies (no `smol`, `async-fs`, etc.)
- Uses Web APIs directly through `wasm-bindgen` and `web-sys`
- Implements concurrency through Web Workers instead of OS threads
- Renders to HTML5 Canvas or WebGPU instead of native graphics APIs

## Architecture

### Core Components

#### Executor (`src/executor.rs`)
- **WebExecutor**: Main executor that uses `wasm-bindgen-futures` for async operations
- **Task**: Future wrapper that provides cancellation and detaching
- **Timer**: Browser-based timer implementation using `setTimeout`
- Time-slicing for long-running tasks to avoid blocking the main thread

### Planned Components

#### Worker Pool (Coming Soon)
- Spawn Web Workers based on `navigator.hardwareConcurrency`
- Message-passing protocol for task distribution
- Serialization of work items using `postcard`

#### Renderer (Coming Soon)
- Canvas 2D for initial implementation
- WebGPU for performance-critical rendering
- Efficient damage tracking and partial redraws

#### Window System (Coming Soon)
- Browser window management
- Focus handling
- Input event processing

## Usage

### Building

```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Build the WASM module
wasm-pack build crates/gpui_web --target web

# Run tests in the browser
wasm-pack test --headless --firefox crates/gpui_web
```

### Basic Example

```rust
use gpui_web::{GpuiWeb, WebExecutor};

// Initialize the platform
let platform = GpuiWeb::new();
let executor = WebExecutor::new();

// Spawn an async task
let task = executor.spawn(async {
    // Your async code here
    println!("Running on the web!");
    42
});

// Await the result
let result = task.await;
assert_eq!(result, 42);
```

### JavaScript Integration

```javascript
import init, { GpuiWeb } from './gpui_web.js';

async function main() {
    // Initialize the WASM module
    await init();

    // Create the platform instance
    const platform = new GpuiWeb();

    // Get hardware info
    const cores = platform.getHardwareConcurrency();
    console.log(`Running with ${cores} cores`);

    // Spawn async tasks
    platform.spawnTask(async () => {
        console.log("Task running!");
    });
}

main();
```

## Design Decisions

### Why a Separate Crate?

The main `gpui` crate has deep dependencies on native-only crates throughout its dependency tree:
- `util` → `smol`, `async-fs`, `which`
- `http_client` → `async-fs`, `util`
- Many transitive dependencies that don't compile to WASM

Creating a separate crate allows us to:
1. Start fresh without the native dependency baggage
2. Design APIs specifically for web constraints
3. Iterate quickly without breaking the native implementation
4. Eventually merge successful patterns back into the main crate

### Concurrency Model

Unlike native platforms, browsers have strict limitations:
- No true blocking operations
- Single-threaded main execution
- Web Workers for parallelism (with message passing only)

Our approach:
1. All UI work stays on the main thread
2. Use time-slicing for CPU-intensive main-thread work
3. Offload heavy computation to Web Workers when possible
4. Serialize tasks for worker execution using `postcard`

### Rendering Strategy

Starting with Canvas 2D because:
- Universal browser support
- Simpler to implement initially
- Good enough for UI rendering

Future WebGPU support for:
- Better performance
- GPU-accelerated effects
- Lower power consumption

## Current Status

✅ **Completed**
- Basic executor with spawn/spawn_background
- Task abstraction with cancellation
- Timer implementation
- WASM module setup

🚧 **In Progress**
- Worker pool implementation
- Message passing protocol

📋 **Planned**
- Canvas renderer
- Window management
- Input handling
- Fetch-based HTTP client
- Storage abstractions

## Development

### Testing

```bash
# Run all tests in headless browser
wasm-pack test --headless --firefox crates/gpui_web

# Run specific test
wasm-pack test --headless --firefox crates/gpui_web -- --test executor

# Run with console output
RUST_LOG=debug wasm-pack test --firefox crates/gpui_web
```

### Debugging

The crate includes `console_error_panic_hook` for better panic messages in the browser console. Enable debug logging with:

```rust
console_log::init_with_level(log::Level::Debug).unwrap();
```

### Performance Profiling

Use browser DevTools:
1. Performance tab for timeline analysis
2. Memory tab for heap snapshots
3. Network tab for WASM loading

## Contributing

When adding new features:

1. Keep dependencies minimal and web-compatible
2. Use `#[wasm_bindgen]` for JavaScript interop
3. Implement time-slicing for long operations
4. Write browser-based tests using `wasm-bindgen-test`
5. Document Web API usage and browser compatibility

## License

Licensed under the Apache License, Version 2.0. See the main Zed repository for details.
