struct Sensor {
    active: bool,
    latest: u32,
}
fn main() {
    let mut sensor = Sensor {
        active: false,
        latest: 0,
    };
    sensor.latest = 42;
    println!("active = {}, latest = {}", sensor.active, sensor.latest);
}
