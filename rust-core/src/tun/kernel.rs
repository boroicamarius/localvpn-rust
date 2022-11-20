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
        log::trace!("stopping KERNEL instance");
        if *self.running.lock().unwrap() == false {
            log::trace!("RUNNING is false");
            return;
        }
        log::trace!("Can stop something");
        *self.terminate.lock().unwrap() = true;
        match self.thread.lock().unwrap().take() {
            Some(thread) => match thread.join() {
                Ok(_) => log::trace!("thread returned with SUCCESS"),
                Err(_) => log::trace!("thread FAILED to return"),
            },
            None => log::trace!("No thread to WAIT"),
        }
    }
    pub fn start_instance(&mut self, fd: i32) -> Result<(), &'static str> {
        if fd < 0 {
            return Err("File descriptor invalid");
        };
        if *self.running.lock().unwrap() {
            self.stop_instance();
        }
        *self.terminate.lock().unwrap() = false;
        let terminate_copy = Arc::clone(&self.terminate);
        *self.running.lock().unwrap() = true;
        *self.thread.lock().unwrap() = Some(thread::spawn(move || {
            KernelLoop::new().run(fd, terminate_copy)
        }));
        return Ok(());
    }
}
