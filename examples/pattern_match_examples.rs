struct SingleTuple(i32);

struct MultipleTuple(i32, String);

struct Struct {
    number: i32,
    identifier: String,
}

fn print_single_tuple(SingleTuple(number): SingleTuple) {
    println!("SingleTuple: {}", number);
}

fn print_multiple_tuple(MultipleTuple(number, identifier): MultipleTuple) {
    println!("MultipleTuple: {}, {}", number, identifier);
}

fn print_struct(Struct { number, identifier }: Struct) {
    println!("Struct: {}, {}", number, identifier);
}

fn main() {
    print_single_tuple(SingleTuple(42));

    print_multiple_tuple(MultipleTuple(42, "foo".into()));

    print_struct(Struct {
        number: 42,
        identifier: "foo".into(),
    });
}
