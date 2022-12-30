# gdbg

Provides the [1] missing [`dbg!`](https://doc.rust-lang.org/std/macro.dbg.html) macro for Gear smart contracts.

## Example

```
use gdbg::dbg;

#[no_mangle]
extern "C" fn init(){
  let payload = String::from_utf8(msg::load_bytes().expect("Failed to load a message"))
    .expect("Invalid init message");
  dbg!(&payload);
}
```

## References

- [1] https://docs.gear.rs/gstd/prelude/index.html#macros

<!-- cargo publish --no-verify --allow-dirty -->
