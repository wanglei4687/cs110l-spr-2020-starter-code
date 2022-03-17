fn main() {
    // variable binding
    let n: i32 = 1;

    // binding with type inference
    let n = 1;

    // variable shadowing(a la OCaml)
    let n = n + 1;

    // mutable values
    let mut n = 0;
    n = n + 1;

    // string literals are immutable pointers
    let s: &str = "Hello World";

    // string objects you can mutate
    let mut s: String = String::from("Hello ");
    s.push_str("world!");

    // Named product type (struct) type definition
    struct Point { x: f32, y: f32 }

    // struct constructor
    let p: Point = Point { x: 1.0, y: 2.0 };

    // struct accessor.
    println!("({}, {})", p.x, p.y);

    // polymorphic named sum type (i.e. variant) or "eum' in Rust
    enum Option<T> { None, Some(T) }

    // enum constructor
    let opt: Option<i32> = Option::Some(1);

    // match statements
    println!("{}", match opt {
	Option::Some(n) => n,
	Option::None  => -1
    });

    // resizable array (vector)
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(3);

    // fixed-size arrays
    let mut arr: [i32; 4]  = [0, 2, 4, 8];
    arr[0] = -2;
    println!("{}", arr[0] + arr[1]);

    // slices (views into arrays/vectors
    let mut slice: &[i32] = &arr[1..];
    println!("{}", slice.iter().sum::<i32>());


    // iterators
    for i in v.iter() {
	println!("{}", i);
    }

    // infinite loops (while true)
    let mut i = 0;
    loop {
	i += 1;
	if i == 10 { break; }
    }

    // while loops
    while i < 20 {
	i += 1;
    }
}

// functions
fn fib(n: i32) -> i32 {
    if n <= 1 { n } else { fib(n-1) + fib(n-2) }
}
