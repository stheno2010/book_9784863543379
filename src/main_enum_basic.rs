enum Type {
    Int,
    Float,
    Boolean,
}
fn print_type(t: Type) {
    match t {
        Type::Int => println!("type is integer"),
        Type::Float => println!("type is floating point"),
        Type::Boolean => println!("type is boolean"),
    }
}
fn main() {
    print_type(Type::Int);
    print_type(Type::Float);
    print_type(Type::Boolean);
}
