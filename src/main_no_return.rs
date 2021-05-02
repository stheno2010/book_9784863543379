
fn main() {
    let x = double(5);
    println!("5 x 2 = {}", x);
    let y = double(4294967294);
    println!("4294967294 x 2 = {}", y);
}
/// Doubles the number given.
///
/// # Examples
///
/// ```
/// let result = docs::double(5);
/// println!("result = {}", result);
/// ```
pub fn double(number: u32) -> u32 {
    if number > (u32::MAX / 2) {
        return u32::MAX;
    }
    number * 2
}
