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
fn use_sensor(s: &mut Sensor) {
    s.latest = 42;
}
fn main() {
    let mut sensor = Sensor::new();
    use_sensor(&mut sensor);
    println!("sensor.latest = {}, sensor.active = {}", sensor.active, sensor.latest);
}
