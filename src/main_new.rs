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
    fn read(&self) -> u32 {
        self.latest
    }
    fn init(&mut self) {
        self.active = true;
        self.latest = 42;
    }
}
fn main() {
    let mut sensor = Sensor::new();
    let latest = sensor.read();
    println!("active = {}, latest = {}", sensor.active, latest);
    sensor.init();
    println!("active = {}, latest = {}", sensor.active, latest);
}
