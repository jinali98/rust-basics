fn main() {
    // tuples
    let tuple_set = (-50, 1.8, 8);

    // read tuple values using index
    let tup_x = tuple_set.0;
    let tup_y = tuple_set.1;
    let tup_z = tuple_set.2;

    // tuple destructuring
    let (x, y, z) = tuple_set;

    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");
    println!("{tup_x}");

    let empty_tuple: () = ();

    // Arrays
    let a = [4, 5, 6];

    let a_0 = a[0];
    println!("array a 0th index value : {a_0}");
    // let a_10 = a[10];
    // println!("array a 10th index value : {a_10}");
}
