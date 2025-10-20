//! Example demonstrating the WebExecutor for async operations in the browser
//!
//! This example shows how to:
//! - Spawn async tasks on the main thread
//! - Use timers for delayed execution
//! - Handle concurrent tasks
//! - Detach tasks for fire-and-forget operations

use gpui_web::{Task, Timer, WebExecutor};
use std::time::Duration;
use wasm_bindgen::prelude::*;

/// Main entry point for the WebAssembly module
#[wasm_bindgen]
pub async fn run_executor_demo() {
    // Initialize logging
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");

    log::info!("Starting WebExecutor demo");

    // Create a new executor
    let executor = WebExecutor::new();

    // Example 1: Simple task spawning
    demo_basic_spawn(&executor).await;

    // Example 2: Multiple concurrent tasks
    demo_concurrent_tasks(&executor).await;

    // Example 3: Timer usage
    demo_timers(&executor).await;

    // Example 4: Task detaching
    demo_detached_tasks(&executor);

    // Example 5: Background work with time-slicing
    demo_background_work(&executor);

    log::info!("WebExecutor demo complete!");
}

/// Demonstrates basic task spawning
async fn demo_basic_spawn(executor: &WebExecutor) {
    log::info!("Demo 1: Basic task spawning");

    // Spawn a simple async task
    let task = executor.spawn(async {
        log::debug!("Task started");

        // Simulate some async work
        Timer::after(Duration::from_millis(100)).await;

        log::debug!("Task completed");
        42
    });

    // Await the result
    let result = task.await;
    log::info!("Task result: {}", result);
}

/// Demonstrates multiple concurrent tasks
async fn demo_concurrent_tasks(executor: &WebExecutor) {
    log::info!("Demo 2: Concurrent tasks");

    // Spawn multiple tasks that run concurrently
    let task1 = executor.spawn(async {
        log::debug!("Task 1 starting");
        Timer::after(Duration::from_millis(200)).await;
        log::debug!("Task 1 complete");
        "First"
    });

    let task2 = executor.spawn(async {
        log::debug!("Task 2 starting");
        Timer::after(Duration::from_millis(100)).await;
        log::debug!("Task 2 complete");
        "Second"
    });

    let task3 = executor.spawn(async {
        log::debug!("Task 3 starting");
        Timer::after(Duration::from_millis(150)).await;
        log::debug!("Task 3 complete");
        "Third"
    });

    // Wait for all tasks to complete
    let result1 = task1.await;
    let result2 = task2.await;
    let result3 = task3.await;

    log::info!("All tasks complete: {}, {}, {}", result1, result2, result3);
}

/// Demonstrates timer usage
async fn demo_timers(executor: &WebExecutor) {
    log::info!("Demo 3: Timer usage");

    // Create a sequence of timed events
    let task = executor.spawn(async {
        log::debug!("Starting timed sequence");

        for i in 1..=3 {
            Timer::after(Duration::from_millis(100 * i as u64)).await;
            log::debug!("Timer {} fired", i);
        }

        "Timers complete"
    });

    let result = task.await;
    log::info!("{}", result);
}

/// Demonstrates detached tasks (fire-and-forget)
fn demo_detached_tasks(executor: &WebExecutor) {
    log::info!("Demo 4: Detached tasks");

    // Spawn a task and immediately detach it
    executor
        .spawn(async {
            log::debug!("Detached task starting");
            Timer::after(Duration::from_millis(500)).await;
            log::debug!("Detached task complete (running in background)");
        })
        .detach();

    // Spawn a task that might fail and log any errors
    executor
        .spawn(async {
            log::debug!("Task with error handling starting");
            Timer::after(Duration::from_millis(200)).await;

            // Simulate potential error condition
            if web_sys::js_sys::Math::random() > 0.5 {
                log::warn!("Simulated error condition");
            } else {
                log::debug!("Task succeeded");
            }
        })
        .detach_and_log_err();

    log::info!("Detached tasks spawned (running in background)");
}

/// Demonstrates background work with time-slicing
fn demo_background_work(executor: &WebExecutor) {
    log::info!("Demo 5: Background work with time-slicing");

    // Schedule CPU-intensive work to be done in time slices
    for i in 0..5 {
        executor.schedule_background_work(move || {
            log::debug!("Background work item {} processing", i);

            // Simulate some CPU-intensive work
            let mut sum = 0u64;
            for j in 0..10000 {
                sum += j;
            }

            log::debug!("Background work item {} complete (sum: {})", i, sum);
        });
    }

    log::info!("Background work scheduled");
}

/// Example of creating a ready task
#[wasm_bindgen]
pub fn demo_ready_task() {
    log::info!("Demo: Ready task");

    // Create a task that's immediately ready with a value
    let task = Task::ready(100);

    // The task is already complete, so we can use it immediately
    let executor = WebExecutor::new();
    let result = executor.block_on(task);

    log::info!("Ready task value: {}", result);
}

/// Example of task chaining
#[wasm_bindgen]
pub async fn demo_task_chaining() {
    log::info!("Demo: Task chaining");

    let executor = WebExecutor::new();

    // Chain multiple async operations
    let final_result = executor
        .spawn(async {
            log::debug!("Step 1: Initial value");
            10
        })
        .await;

    let final_result = executor
        .spawn(async move {
            log::debug!("Step 2: Double the value");
            Timer::after(Duration::from_millis(100)).await;
            final_result * 2
        })
        .await;

    let final_result = executor
        .spawn(async move {
            log::debug!("Step 3: Add 5");
            Timer::after(Duration::from_millis(100)).await;
            final_result + 5
        })
        .await;

    log::info!("Final chained result: {}", final_result);
}

/// Example showing executor reuse
#[wasm_bindgen]
pub struct ExecutorHandle {
    executor: std::sync::Arc<WebExecutor>,
}

#[wasm_bindgen]
impl ExecutorHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_log::init_with_level(log::Level::Debug).ok();
        Self {
            executor: std::sync::Arc::new(WebExecutor::new()),
        }
    }

    /// Spawn a task that logs a message after a delay
    #[wasm_bindgen]
    pub fn spawn_delayed_log(&self, message: String, delay_ms: u32) {
        let executor = self.executor.clone();

        executor
            .spawn(async move {
                Timer::after(Duration::from_millis(delay_ms as u64)).await;
                log::info!("Delayed message: {}", message);
            })
            .detach();
    }

    /// Run a computation and return the result
    #[wasm_bindgen]
    pub async fn compute(&self, input: i32) -> i32 {
        let executor = self.executor.clone();

        executor
            .spawn(async move {
                log::debug!("Computing with input: {}", input);
                Timer::after(Duration::from_millis(50)).await;

                let result = input * 2 + 10;
                log::debug!("Computation result: {}", result);

                result
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_executor_demo() {
        run_executor_demo().await;
    }

    #[wasm_bindgen_test]
    fn test_ready_task() {
        demo_ready_task();
    }

    #[wasm_bindgen_test]
    async fn test_task_chaining() {
        demo_task_chaining().await;
    }
}
