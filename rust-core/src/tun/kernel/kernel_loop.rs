use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};
use std::sync::{Arc, Mutex};

pub struct KernelLoop;

impl KernelLoop {
    fn default() -> Self {
        Self {}
    }

    pub fn new() -> Self {
        Self {
            ..KernelLoop::default()
        }
    }

    pub fn run(&mut self, file_descriptor: i32, to_terminate: Arc<Mutex<bool>>) {
        let mut poll: Poll = Poll::new().unwrap();
        let mut events: Events = Events::with_capacity(1024);
        let kernel_token = Token(0);
        poll.registry()
            .register(
                &mut SourceFd(&file_descriptor),
                kernel_token,
                Interest::READABLE | Interest::WRITABLE,
            )
            .unwrap();
        loop {
            {
                log::trace!("checking TERM");
                if *to_terminate.lock().unwrap() {
                    log::trace!("TERM FOUND");
                    break;
                };
            }
            poll.poll(&mut events, None).unwrap();
            for event in &events {
                if event.token() == kernel_token && event.is_readable() {
                    log::trace!("TOKEN writeable");
                }
            }
        }
    }
}
