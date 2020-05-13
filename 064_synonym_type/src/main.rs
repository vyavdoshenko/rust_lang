use std::fmt;

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
    let f: Thunk = Box::new(|| println!("hi"));
    f
}

pub trait LongWrite {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error>;
    fn flush(&mut self) -> std::result::Result<(), std::io::Error>;

    fn write_all(&mut self, buf: &[u8]) -> std::result::Result<(), std::io::Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> std::result::Result<(), std::io::Error>;
}

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait ShortWrite {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}