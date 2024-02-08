fn main() {
    // match is a mix of switch and if
    let x = 3;

    match x {
        1 => println!("x match with 1 "),
        2 => println!("x match with 2"),
        _ => println!("x doesnt match with anything"),
    }

    let y = true;
    let z = false;

    match (z, y) {
        (true, false) => println!("z is true and y is false"),
        (false, true) => println!("z is true and y is false"),
        (_, _) => println!("doesnt match with anything"),
    }
}
