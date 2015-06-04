fn main() {
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x -3;

        //if x % 2 == 0 { continue; }

        println!("{}",x);

        if x % 5 == 0 {
            done = true;
        }

        //if x % 5 == 0 { break; }
    }
}
