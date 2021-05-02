struct Sensor {
    active: bool,
    latest: u32,
}
fn main() {
    let sensor = Sensor {
        active: false,
        latest: 0,
    };
    println!("active = {}, latest = {}", sensor.active, sensor.latest);
}
