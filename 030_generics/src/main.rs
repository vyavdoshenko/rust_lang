struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn sum(&self) -> f32 {
        (self.x + self.y)
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("both_integer.x = {}", both_integer.x());
    println!("both_float.x = {}", both_float.x());
    println!("integer_and_float.x = {}", integer_and_float.x());

    println!("both_float.sum = {}", both_float.sum());

    let mu = both_integer.mixup(both_float);

    println!("mu.x = {}, mu.y = {}", mu.x, mu.y);
}
