```
fn main() {
    dbg!(add(2, 2));

    std::dbg!(add(2, 2));

    // gdbg::dbg is more compact than std::dbg
    dbg!(Point { x: 2, y: 2 });

    std::dbg!(Point { x: 2, y: 2 });
}
```

```
$ cargo r
[examples/example-lib/main.rs:20] add(2, 2) = 4  // gdbg::dbg
[examples/example-lib/main.rs:22] add(2, 2) = 4  // std::dbg
[examples/example-lib/main.rs:25] Point { x: 2, y: 2 } = Point { x: 2, y: 2 }  // gdbg::dbg
[examples/example-lib/main.rs:27] Point { x: 2, y: 2 } = Point {               // std::dbg
    x: 2,
    y: 2,
}
```
