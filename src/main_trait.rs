trait Sensor {
    fn read(&self) -> u32;
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
fn print_sensor_value(sensor: impl Sensor) {
    println!("sensor value = {}", sensor.read());
}
fn main() {
    let light_sensor = LightSensor { value: 42 };
    let temp_sensor = TemperatureSensor { value: 35.5 };

    println!("light sensor value = {}", light_sensor.read());
    println!("temp sensor value = {}", temp_sensor.read());
    print_sensor_value(light_sensor);
    print_sensor_value(temp_sensor);
}
