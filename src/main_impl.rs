struct Sensor {
    active: bool,
    latest: u32,
}
impl Sensor {
    fn read(&self) -> u32 {
        self.latest
    }
    fn init(&mut self) {
        self.active = true;
        self.latest = 42;
    }
}
fn main() {
    let mut sensor = Sensor {
        active: false,
        latest: 0,
    };
    sensor.init();
    let latest = sensor.read();
    println!("latest = {}", latest);
}
