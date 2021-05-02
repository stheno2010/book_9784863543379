struct Sensor {
    active: bool,
    latest: u32,
}
impl Sensor {
    fn new() -> Sensor {
        Sensor {
            active: false,
            latest: 0,
        }
    }
}
fn use_sensor(s: &Sensor) {
    println!("latest = {}", s.latest);
}
fn main() {
    let sensor = Sensor::new();
    use_sensor(&sensor);
    println!("sensor.latest = {}, sensor.active = {}", sensor.active, sensor.latest);
}
