use futures::channel::oneshot;
use futures::{Future, FutureExt};
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

/// Type alias for boxed futures that can be executed on the web platform
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// A task handle that can be used to await or cancel a spawned task
#[derive(Clone)]
pub struct Task<T> {
    inner: Arc<TaskInner<T>>,
}

struct TaskInner<T> {
    receiver: Mutex<Option<oneshot::Receiver<T>>>,
    is_detached: Mutex<bool>,
}

impl<T> Task<T> {
    /// Creates a new task from a oneshot receiver
    fn new(receiver: oneshot::Receiver<T>) -> Self {
        Self {
            inner: Arc::new(TaskInner {
                receiver: Mutex::new(Some(receiver)),
                is_detached: Mutex::new(false),
            }),
        }
    }

    /// Creates a task that is immediately ready with the given value
    pub fn ready(value: T) -> Self
    where
        T: 'static,
    {
        let (tx, rx) = oneshot::channel();
        let _ = tx.send(value);
        Self::new(rx)
    }

    /// Detaches the task, allowing it to run in the background
    pub fn detach(self) {
        *self.inner.is_detached.lock().unwrap() = true;
    }

    /// Detaches the task and logs any errors that occur
    pub fn detach_and_log_err(self)
    where
        T: std::fmt::Debug + 'static,
    {
        let inner = self.inner.clone();
        spawn_local(async move {
            if let Some(rx) = inner.receiver.lock().unwrap().take() {
                if let Err(e) = rx.await {
                    log::error!("Detached task failed: {:?}", e);
                }
            }
        });
        self.detach();
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut receiver = self.inner.receiver.lock().unwrap();
        if let Some(rx) = receiver.as_mut() {
            match rx.poll_unpin(cx) {
                Poll::Ready(Ok(value)) => {
                    *receiver = None;
                    Poll::Ready(value)
                }
                Poll::Ready(Err(_)) => panic!("Task sender dropped"),
                Poll::Pending => Poll::Pending,
            }
        } else {
            panic!("Task polled after completion");
        }
    }
}

/// Web-based executor for running async tasks
pub struct WebExecutor {
    background_queue: Arc<Mutex<VecDeque<BoxedTask>>>,
}

type BoxedTask = Box<dyn FnOnce() + 'static>;

impl WebExecutor {
    /// Creates a new WebExecutor
    pub fn new() -> Self {
        Self {
            background_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Spawns a future on the main thread
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let (tx, rx) = oneshot::channel();

        spawn_local(async move {
            let result = future.await;
            let _ = tx.send(result);
        });

        Task::new(rx)
    }

    /// Spawns a future that will be executed in the background
    /// On web, this still runs on the main thread but uses time-slicing
    pub fn spawn_background<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        // For now, just forward to regular spawn
        // In the future, this could use Web Workers
        self.spawn(future)
    }

    /// Blocks the current task until the given future completes
    /// Note: This doesn't truly block on web, it uses polling
    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        // Create a simple polling executor for web
        let mut future = Box::pin(future);
        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);

        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(result) => return result,
                Poll::Pending => {
                    // In a real web implementation, we'd yield to the browser here
                    // For now, this is a simplified version
                    log::warn!("block_on called in web context - this may not work as expected");
                }
            }
        }
    }

    /// Runs a closure on the main thread
    pub fn run_on_main<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        // On web, everything runs on the main thread already
        f()
    }

    /// Schedules work to be done in the background using time-slicing
    pub fn schedule_background_work<F>(&self, work: F)
    where
        F: FnOnce() + 'static,
    {
        self.background_queue
            .lock()
            .unwrap()
            .push_back(Box::new(work));
        self.process_background_queue();
    }

    /// Processes the background work queue using time-slicing
    fn process_background_queue(&self) {
        let queue = self.background_queue.clone();

        spawn_local(async move {
            const TIME_SLICE_MS: i32 = 5; // 5ms time slices

            let start = performance_now();

            while performance_now() - start < TIME_SLICE_MS as f64 {
                let task = queue.lock().unwrap().pop_front();

                match task {
                    Some(work) => work(),
                    None => break,
                }
            }

            // If there's more work, schedule another time slice
            if !queue.lock().unwrap().is_empty() {
                let window = window().expect("no window");
                let queue_clone = queue.clone();

                let closure = Closure::once(move || {
                    Self::process_remaining_queue(queue_clone);
                });

                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        0,
                    )
                    .expect("set_timeout failed");

                closure.forget();
            }
        });
    }

    fn process_remaining_queue(queue: Arc<Mutex<VecDeque<BoxedTask>>>) {
        spawn_local(async move {
            const TIME_SLICE_MS: i32 = 5;
            let start = performance_now();

            while performance_now() - start < TIME_SLICE_MS as f64 {
                let task = queue.lock().unwrap().pop_front();

                match task {
                    Some(work) => work(),
                    None => break,
                }
            }

            if !queue.lock().unwrap().is_empty() {
                let window = window().expect("no window");
                let queue_clone = queue.clone();

                let closure = Closure::once(move || {
                    Self::process_remaining_queue(queue_clone);
                });

                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        0,
                    )
                    .expect("set_timeout failed");

                closure.forget();
            }
        });
    }
}

impl Default for WebExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer implementation for web platform
pub struct Timer {
    _handle: Option<i32>,
    receiver: oneshot::Receiver<()>,
}

impl Timer {
    /// Creates a new timer that fires after the specified duration
    pub fn after(duration: Duration) -> Self {
        let (tx, rx) = oneshot::channel();
        let millis = duration.as_millis() as i32;

        let window = window().expect("no window");

        let closure = Closure::once(move || {
            let _ = tx.send(());
        });

        let handle = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                millis,
            )
            .expect("set_timeout failed");

        closure.forget();

        Timer {
            _handle: Some(handle),
            receiver: rx,
        }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.receiver.poll_unpin(cx).map(|_| ())
    }
}

/// Creates a Waker that does nothing
fn noop_waker() -> Waker {
    struct NoopWaker;

    impl Wake for NoopWaker {
        fn wake(self: Arc<Self>) {}
        fn wake_by_ref(self: &Arc<Self>) {}
    }

    Arc::new(NoopWaker).into()
}

/// Gets the current time in milliseconds using the Performance API
fn performance_now() -> f64 {
    window()
        .expect("no window")
        .performance()
        .expect("no performance")
        .now()
}

/// A handle for spawning tasks on the web executor
#[derive(Clone)]
pub struct WebSpawner {
    executor: Arc<WebExecutor>,
}

impl WebSpawner {
    /// Creates a new spawner from an executor
    pub fn new(executor: Arc<WebExecutor>) -> Self {
        Self { executor }
    }

    /// Spawns a future on the executor
    pub fn spawn<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        self.executor.spawn(future)
    }

    /// Spawns a future in the background
    pub fn spawn_background<F>(&self, future: F) -> Task<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        self.executor.spawn_background(future)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_spawn() {
        let executor = WebExecutor::new();
        let task = executor.spawn(async { 42 });
        let result = task.await;
        assert_eq!(result, 42);
    }

    #[wasm_bindgen_test]
    async fn test_timer() {
        let timer = Timer::after(Duration::from_millis(10));
        timer.await;
        // If we get here, the timer fired successfully
    }

    #[wasm_bindgen_test]
    fn test_ready_task() {
        let task = Task::ready(100);
        let executor = WebExecutor::new();
        let result = executor.block_on(task);
        assert_eq!(result, 100);
    }
}
