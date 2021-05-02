#[derive(Debug)]
enum Error {
    Overflow,
    Zero,
}
fn double(number: u32) -> Result<u32, Error> {
    if number == 0 {
        Err(Error::Zero)
    } else if number > (u32::MAX / 2) {
        Err(Error::Overflow)
    } else {
        Ok(number * 2)
    }
}
fn func() -> Result<u32, Error> {
    let doubled = double(u32::MAX)?;
    Ok(doubled)
}
fn main() {
    match double(10) {
        Ok(x) => println!("doubled = {}", x),
        Err(_e) => println!("double failed!"),
    }
    if let Ok(x) = double(u32::MAX) {
        println!("doubled = {}", x);
    }
    if let Err(e) = func() {
        println!("func failed with {:?}", e);
    }
}
