fn main() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &mut array[1..3];
    slice[0] = 6;
    println!("slice = {:?}", slice);
    println!("array = {:?}", array);
}
