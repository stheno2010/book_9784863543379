trait Sensor {
    fn read(&self) -> u32;
    fn fill(&self, buffer: &mut [u32]) {
        for element in buffer.iter_mut() {
            *element = self.read();
        }
    }
}
struct LightSensor {
    value: u32,
}
impl Sensor for LightSensor {
    fn read(&self) -> u32 {
        self.value
    }
}
struct TemperatureSensor {
    value: f32,
}
impl Sensor for TemperatureSensor {
    fn read(&self) -> u32 {
        self.value as u32
    }
}
fn main() {
    let light_sensor = LightSensor { value: 42 };
    let temp_sensor = TemperatureSensor { value: 35.5 };

    let mut buf = [0u32; 4];
    light_sensor.fill(&mut buf);
    println!("buf = {:?}", buf);

    let mut buf = [0u32; 4];
    temp_sensor.fill(&mut buf);
    println!("buf = {:?}", buf);
}
