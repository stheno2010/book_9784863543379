fn multi_life<'a, 'b>(v1: &'a Vec<i32>, v2: &'b Vec<i32>) -> (&'a i32, &'b i32) {
    (&v1[0], &v2[0])
}
fn main() {
    let v1 = vec![0, 1, 2];
    let r1;
    {
        let v2 = vec![3, 4, 5];
        let r2;
        {
            let r = multi_life(&v1, &v2);
            r1 = r.0;
            r2 = r.1;
        }
        println!("r2 = {:?}", r2);
    }
    println!("r1 = {:?}", r1);
}
