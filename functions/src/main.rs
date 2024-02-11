fn main() {
    println!("Hello, world!");
    let d = another_functions(8, 'B');
}

fn another_functions(value: i32, word: char) -> i32 {
    let x = {
        let y = 5;
        println!("value of y is:  {}", y);
        y + 1
    };

    println!("{}, {}, {}", value, z, x);
}
