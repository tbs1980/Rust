fn main()
{
    let x = true;

    let y:bool = false;

    let xx = 'x';

    //Unlike some other languages, this means that Rust’s char is not a single byte, but four.
    let two_hearts = '💕';

    let a = 10; // x has type i32

    let b = 1.0; // y has type f64

    // arrays
    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]

    let a1 = [0; 20]; // a: [i32; 20]

    println!("a1 has {} elements", a1.len());

    let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]

    println!("The second name is: {}", names[1]);

    //slices
    let middle = &a1[1..4];

    //tuples
    let x1: (i32, &str) = (1, "hello");

    let mut x2 = (1, 2);
    let y2 = (2,3);

    x2 = y2;

    //Functions
    fn foo(x: i32) -> i32 { x }
    let x3: fn(i32) -> i32 = foo;

}
