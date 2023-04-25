# gdbg

Provides the [1] missing [`dbg!`](https://doc.rust-lang.org/std/macro.dbg.html)
macro for Gear smart contracts.

You can see the debug messages when running the program using the `gtest` crate.
To see these messages when executing the program on the node, you should run the
node with the `RUST_LOG="gwasm=debug"` environment variable.

## Example

[examples/example/lib.rs](examples/example/lib.rs)

```
#![no_std]

use gdbg::dbg;
use gstd::prelude::String;

#[no_mangle]
extern "C" fn init() {
    let payload = String::from_utf8(gstd::msg::load_bytes().expect("Failed to load a message"))
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

        let res = program.send_bytes(42, "Let's start");
        assert!(res.log().is_empty());
    }
}
```

run `cargo test`

```
...
[DEBUG tests::it_works] [/home/btwiuse/gdbg/examples/example/lib.rs:10] &payload = "Let's start"
test tests::it_works ... ok
...
```

## References

- [1] https://docs.gear.rs/gstd/prelude/index.html#macros

<!-- cargo publish --no-verify --allow-dirty -->
