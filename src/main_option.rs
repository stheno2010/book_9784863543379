fn main() {
    match double(10) {
        Some(x) => println!("doubled: {}", x),
        None => println!("double failed"),
    }
    match double(u32::MAX) {
        Some(x) => println!("doubled: {}", x),
        None => println!("double failed"),
    }
    if let Some(x) = double(10) {
        println!("doubled: {}", x);
    }
    if let Some(x) = double(u32::MAX) {
        println!("doubled: {}", x);
    }
}
fn double(number: u32) -> Option<u32> {
    if number > (u32::MAX / 2) {
        return None;
    }
    Some(number * 2)
}
