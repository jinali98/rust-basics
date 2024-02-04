fn main() {
    // tuples
    let tuple_set = (-50, 1.8, 8);
    let tup_x = tuple_set.0;
    let tup_y = tuple_set.1;
    let tup_z = tuple_set.2;

    let (x, y, z) = tuple_set;

    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");
    println!("{tup_x}");
}
