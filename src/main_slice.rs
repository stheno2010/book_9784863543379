fn main() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &array[1..3];
    println!("slice = {:?}", slice);
    println!("slice[0] = {}", slice[0]);
}
