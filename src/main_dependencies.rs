use oorandom::Rand32;
fn main() {
    let seed = 1;
    let mut rng = Rand32::new(seed);
    println!("my number = {}", rng.rand_i32());
}
