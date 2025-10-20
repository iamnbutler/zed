//! GPUI Web Platform Implementation
//!
//! This crate provides a web-compatible implementation of GPUI's core functionality,
//! designed to run in WebAssembly environments without native OS dependencies.
//!
//! ## Architecture
//!
//! Unlike the native GPUI implementation, this crate:
//! - Uses `wasm-bindgen-futures` instead of `smol` for async operations
//! - Implements time-slicing for long-running tasks on the main thread
//! - Will eventually use Web Workers for true parallelism
//! - Renders to HTML5 Canvas or WebGPU instead of native graphics APIs
//!
//! ## Current Status
//!
//! This is a work in progress. Currently implemented:
//! - Basic executor with main-thread task spawning
//! - Timer implementation using browser setTimeout
//! - Task abstraction compatible with GPUI's API
//!
//! ## Future Work
//!
//! - Web Worker pool for background task execution
//! - Canvas/WebGPU rendering backend
//! - Window management through browser APIs
//! - Fetch-based HTTP client
//! - IndexedDB/LocalStorage for persistence

use wasm_bindgen::prelude::*;

pub mod demo;
pub mod executor;
pub mod renderer;

// Re-export main types for convenient access
pub use demo::QuadDemo;
pub use executor::{Task, Timer, WebExecutor, WebSpawner};
pub use renderer::WebRenderer;

/// Initialize the web platform
///
/// This should be called once at the start of your application to set up
/// panic handling and logging for the web environment.
#[wasm_bindgen(start)]
pub fn initialize() {
    // Set up panic hook for better error messages in the browser console
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Initialize console logging
    #[cfg(feature = "console_log")]
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");

    log::info!("GPUI Web Platform initialized");
}

/// Entry point for WebAssembly module
///
/// This is called when the WASM module is loaded in the browser.
/// It returns a handle to the web platform that can be used to
/// spawn tasks and interact with the browser environment.
#[wasm_bindgen]
pub struct GpuiWeb {
    executor: std::sync::Arc<WebExecutor>,
}

#[wasm_bindgen]
impl GpuiWeb {
    /// Creates a new instance of the GPUI web platform
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        initialize();

        Self {
            executor: std::sync::Arc::new(WebExecutor::new()),
        }
    }

    /// Spawns an async task that runs on the main thread
    ///
    /// Note: This is exposed for JavaScript interop. Rust code should
    /// use the executor directly.
    #[wasm_bindgen(js_name = spawnTask)]
    pub fn spawn_task(&self, callback: js_sys::Function) -> Result<(), JsValue> {
        let executor = self.executor.clone();

        executor
            .spawn(async move {
                // Convert JS function to a future
                let promise = callback.call0(&JsValue::NULL)?;
                let future = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(promise));
                future.await?;
                Ok::<(), JsValue>(())
            })
            .detach();

        Ok(())
    }

    /// Gets the number of hardware threads available
    ///
    /// This uses navigator.hardwareConcurrency to determine how many
    /// Web Workers we should spawn for parallel execution.
    #[wasm_bindgen(js_name = getHardwareConcurrency)]
    pub fn get_hardware_concurrency(&self) -> u32 {
        web_sys::window()
            .map(|w| w.navigator().hardware_concurrency())
            .unwrap_or(4.0) as u32
    }

    /// Logs a message to the browser console
    #[wasm_bindgen(js_name = log)]
    pub fn log(&self, message: &str) {
        log::info!("{}", message);
    }
}

impl Default for GpuiWeb {
    fn default() -> Self {
        Self::new()
    }
}

// Platform detection utilities
#[cfg(target_arch = "wasm32")]
pub fn is_web() -> bool {
    true
}

#[cfg(not(target_arch = "wasm32"))]
pub fn is_web() -> bool {
    false
}

/// Future modules will be added here as we build them out:
// pub mod window;      // Window management integration
// pub mod worker_pool; // Web Worker pool for parallelism
// pub mod http;        // Fetch API wrapper
// pub mod storage;     // IndexedDB/LocalStorage wrapper
// pub mod input;       // Browser input event handling

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_platform_detection() {
        assert!(is_web());
    }

    #[wasm_bindgen_test]
    fn test_initialization() {
        let platform = GpuiWeb::new();
        assert!(platform.get_hardware_concurrency() > 0);
    }

    #[wasm_bindgen_test]
    async fn test_executor_integration() {
        let platform = GpuiWeb::new();
        let executor = platform.executor.clone();

        let result = executor.spawn(async { 1 + 1 }).await;

        assert_eq!(result, 2);
    }
}
