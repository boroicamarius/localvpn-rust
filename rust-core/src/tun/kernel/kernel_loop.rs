use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};
use std::sync::{Arc, Mutex};
use std::{
    fs::File,
    io::{self, Read, Write},
    os::unix::io::FromRawFd,
};
#[path = "../packet_system/mod.rs"]
mod packet_system;

use packet_system::ip_packet::IpPacket;

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
        let mut tun_fd = unsafe { File::from_raw_fd(file_descriptor.to_owned()) };
        poll.registry()
            .register(
                &mut SourceFd(&file_descriptor),
                kernel_token,
                Interest::READABLE | Interest::WRITABLE,
            )
            .unwrap();
        loop {
            {
                if *to_terminate.lock().unwrap() {
                    log::trace!("TERM FOUND");
                    break;
                };
            }
            poll.poll(&mut events, None).unwrap();
            for event in &events {
                if event.token() == kernel_token && event.is_readable() {
                    let mut buffer: [u8; 65535] = [0; 65535];

                    match tun_fd.read(&mut buffer) {
                        Ok(count) => {
                            let version = buffer[0].checked_shr(4).unwrap();
                            if version == 4 {}
                            // else{

                            // }
                        }
                        Err(error) => {
                            log::trace!("Error at KLOOP: {}", error)
                        }
                    }
                    log::trace!("TOKEN writeable");
                }
            }
        }
    }
}
