mod kernel_loop;
use kernel_loop::KernelLoop;
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct Kernel {
    thread: Mutex<Option<thread::JoinHandle<()>>>,
    terminate: Arc<Mutex<bool>>,
    running: Mutex<bool>,
}

impl Kernel {
    fn default() -> Self {
        Self {
            thread: Mutex::new(None),
            terminate: Arc::new(Mutex::new(false)),
            running: Mutex::new(false),
        }
    }
    pub fn new() -> Self {
        Self {
            ..Kernel::default()
        }
    }
    pub fn stop_instance(&mut self) {
        if *self.running.lock().unwrap() == false {
            return;
        }
        *self.terminate.lock().unwrap() = true;
        if let Some(thread) = self.thread.lock().unwrap().take() {
            match thread.join() {
                Ok(_) => log::trace!("thread returned with SUCCESS"),
                Err(_) => log::trace!("thread FAILED to return"),
            }
        }
    }
    pub fn start_instance(&mut self, fd: i32) -> Result<(), &'static str> {
        if fd < 0 {
            return Err("File descriptor invalid");
        };
        if *self.running.lock().unwrap() {
            self.stop_instance();
        }
        let terminate_copy = Arc::clone(&self.terminate);
        *self.thread.lock().unwrap() = Some(thread::spawn(move || {
            KernelLoop::new().run(fd, terminate_copy)
        }));
        return Ok(());
    }
}
