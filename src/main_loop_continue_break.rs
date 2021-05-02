fn main() {
    let mut count = 0;
    loop {
        count += 1;
        if count %2 == 1 {
            println!("odd");
            continue;
        }
        println!("even");
        if count == 4 {
            break;
        }
    }
}
