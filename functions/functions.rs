fn print_number(x: i32)
{
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

/*
This reveals two interesting things about Rust: it is an expression-based language, and semicolons are different from semicolons in other ‘curly brace and semicolon’-based languages. These two things are related.
 */
fn add_one(x: i32) -> i32 {
    x + 1 //no semi-colon here
}

fn main ()
{
    let x: i32 = 40;

    print_number(x);

    print_sum(5, 6);

    let x: i32 = add_one(x);

    print_number(x);
}
