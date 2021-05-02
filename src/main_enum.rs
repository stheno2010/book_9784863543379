enum Type {
    Int(i64),
    Float(f64),
    Boolean(bool),
}
fn print_type(t: Type) {
    match t {
        Type::Int(i) => println!("integer value: {}", i),
        Type::Float(f) => println!("floating point value: {}", f),
        Type::Boolean(b) => println!("boolean value: {}", b),
    }
}
fn main() {
    print_type(Type::Int(42));
    print_type(Type::Float(4.2));
    print_type(Type::Boolean(false));
}
