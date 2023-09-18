#![no_std]

use gdbg::dbg;
use gstd::prelude::String;

#[no_mangle]
extern "C" fn init() {
    dbg!(gstd::msg::source());
    let payload = dbg!(String::from_utf8(
        dbg!(gstd::msg::load_bytes()).expect("Failed to load a message")
    ))
    .expect("Invalid init message");
    dbg!(&payload);
}

#[cfg(test)]
mod tests {
    use gtest::{Program, System};

    #[test]
    fn it_works() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(42, "INIT");
    }
}
