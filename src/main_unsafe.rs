static mut X: i32 = 42;
fn main() {
    unsafe {
        println!("X = {}", X);
    }
    unsafe {
        X += 1;
        println!("X = {}", X);
    }
}
