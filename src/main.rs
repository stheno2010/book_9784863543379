#[derive(Debug)]
struct Sensor {
    active: bool,
    latest: u32,
}
fn main() {
    let s = Sensor {
        active: true,
        latest: 42,
    };
    println!("{:?}", s);
}
