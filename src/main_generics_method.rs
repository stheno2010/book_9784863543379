struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point{ x, y }
    }
    fn x(&self) -> &T {
        &self.x
    }
}
fn make_point<T>(x: T, y: T) -> Point<T> {
    Point{ x, y }
}
fn main() {
    let p_i32 = make_point::<i32>(300, 400);
    let p_i8 = make_point::<i32>(10, 20);
    let p = make_point(1, 2);
    let p1 = Point::new(11, 22);

    use std::mem::size_of_val;
    println!("size of x in p_i32 = {}", size_of_val(&p_i32.x));
    println!("size of y in p_i8 = {}", size_of_val(&p_i8.y));
    println!("size of y in p = {}", size_of_val(&p.y));
    println!("p1.x = {}", p1.x());
}
