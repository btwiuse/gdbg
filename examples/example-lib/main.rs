use example_lib::add;
use gdbg::dbg;
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

fn main() {
    dbg!(add(2, 2));

    std::dbg!(add(2, 2));

    // gdbg::dbg is more compact than std::dbg
    dbg!(Point { x: 2, y: 2 });

    std::dbg!(Point { x: 2, y: 2 });
}
