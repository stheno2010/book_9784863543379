fn main() {
    let x: i32 = 42;
    let y: i64 = x.into();
    let z = i64::from(x);
    let w = x as i64;
    println!("y = {}", y);
    println!("z = {}", z);
    println!("w = {}", w);

    use std::convert::TryInto;
    let x: i64 = 42;
    let y: i32 = x.try_into().unwrap();
    println!("y = {}", y);
}
