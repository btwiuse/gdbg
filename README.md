# gdbg

Provides the [1] compact version of [`dbg!`](https://doc.rust-lang.org/std/macro.dbg.html)
macro for [2] gstd and std.

Unlike `std::dbg`, `gdbg::dbg` uses `{:?}` as formatting which is more compact than `{:#?}`

You can see the debug messages when running the program using the `gtest` crate.
To see these messages when executing the program on the node, you should run the
node with the `RUST_LOG="gwasm=debug"` environment variable.

## Example Contract (no_std)

```
[dependeicies]
gstd = { version = "1" }
gdbg = { version = "0.1" }
```

[examples/example-contract/lib.rs](examples/example-contract/lib.rs)

```
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

fn add(left: u32, right: u32) -> u32 {
    left + right
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
        assert!(res.log().is_empty());
    }
}
```

run `cargo test`

```
...
running 1 test
[DEBUG tests::it_works] [/home/navigaid/gdbg/examples/example/lib.rs:7] 2 = 2
[DEBUG tests::it_works] [/home/navigaid/gdbg/examples/example/lib.rs:8] gstd::msg::source() = 0x2a00000000000000000000000000000000000000000000000000000000000000
[DEBUG tests::it_works] [/home/navigaid/gdbg/examples/example/lib.rs:9] gstd::msg::load_bytes() = Ok([ 73, 78, 73, 84 ])
[DEBUG tests::it_works] [/home/navigaid/gdbg/examples/example/lib.rs:9] String::from_utf8(dbg!(gstd::msg::load_bytes()).expect("Failed to load a message")) = Ok("INIT")
[DEBUG tests::it_works] [/home/navigaid/gdbg/examples/example/lib.rs:11] &payload = "INIT"                                                                                    
test tests::it_works ... ok
...
```

## Example Binary (std)

```
[dependeicies]
gdbg = { version = "0.1", features = ["std"] }
```

[examples/example-binary/main.rs](examples/example-binary/main.rs)


```
fn main() {
    gdbg::dbg!(add(2, 2));

    std::dbg!(add(2, 2));

    // gdbg::dbg prints more compact result than std::dbg
    gdbg::dbg!(Point { x: 2, y: 2 });

    std::dbg!(Point { x: 2, y: 2 });
}
```

run `cargo r`

```
[examples/example-lib/main.rs:20] add(2, 2) = 4                                // gdbg::dbg
[examples/example-lib/main.rs:22] add(2, 2) = 4                                // std::dbg
[examples/example-lib/main.rs:25] Point { x: 2, y: 2 } = Point { x: 2, y: 2 }  // gdbg::dbg
[examples/example-lib/main.rs:27] Point { x: 2, y: 2 } = Point {               // std::dbg
    x: 2,
    y: 2,
}
```

## References

- [1] https://github.com/rust-lang/rust/issues/82778 
- [2] https://docs.gear.rs/gstd/prelude/index.html#macros

<!-- cargo publish --no-verify --allow-dirty -->
