mod tun;

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate android_logger;
    extern crate jni;
    extern crate log;

    use self::jni::JNIEnv;
    use super::*;
    use android_logger::Config;
    use jni::objects::JObject;
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    use tun::kernel::Kernel;

    lazy_static! {
        pub static ref TUN_KERNEL: Mutex<Option<Kernel>> = Mutex::new(None);
    }

    #[no_mangle]
    pub extern "system" fn Java_com_training_project_LocalVpn_create_1native(
        _env: JNIEnv,
        _class: JObject,
        _object: JObject,
    ) {
        android_logger::init_once(
            Config::default()
                .with_tag("LocalVpn")
                .with_min_level(log::Level::Trace),
        );
        log::trace!("Native Created!");
        let mut kernel = TUN_KERNEL
            .lock()
            .expect("Could not lock kernel at NATIVE init");
        *kernel = Some(Kernel::new());
        match *kernel {
            Some(_) => log::trace!("Kernel initialized"),
            None => log::trace!("Kernel uninitialized"),
        }
    }
    #[no_mangle]
    pub extern "system" fn Java_com_training_project_LocalVpn_start_1vpn(
        _env: JNIEnv,
        _: JObject,
        fd: i32,
    ) {
        let mut kist = TUN_KERNEL.lock().unwrap();
        match kist.as_mut() {
            Some(kernel) => match kernel.start_instance(fd) {
                Ok(_) => log::trace!("KIST started SUCCESSFULLY"),
                Err(_) => log::trace!("KIST failed to start"),
            },
            None => {
                *kist = Some(Kernel::new());
                if let Some(kernel) = kist.as_mut() {
                    match kernel.start_instance(fd) {
                        Ok(_) => log::trace!("created KIST & started SUCCESSFULLY"),
                        Err(_) => log::trace!("created KIST & failed to start"),
                    }
                }
            }
        }
    }
    #[no_mangle]
    pub extern "system" fn Java_com_training_project_LocalVpn_stop_1vpn(_env: JNIEnv, _: JObject) {
        let mut kist = TUN_KERNEL.lock().unwrap();
        match kist.as_mut() {
            Some(kernel) => {
                kernel.stop_instance();
                log::trace!("kernel STOPPED")
            }
            None => log::trace!("kernel is NONE so nothing is done"),
        }
    }
}
