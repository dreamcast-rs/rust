enum Foo {
    A = 1i64,
    //~^ ERROR mismatched types
    //~| NOTE expected `isize`, found `i64`
    B = 2u8
    //~^ ERROR mismatched types
    //~| NOTE expected `isize`, found `u8`
}

fn main() {}
