fn main() {
    //   control flow

    let num = 11;

    if num == 1 {
        println!("you selected 1");
    } else if num == 2 {
        println!("you selected 2");
    } else {
        println!("you didnt select any");
    }

    // loops

    let mut x = 1;

    loop {
        x = x * 2;

        if x > 5000 {
            break;
        }
        println!("the value of x is: {}", x);
    }

    let mut y = 0;

    while y < 10 {
        println!("the value of y is: {}", y);
        y += 1;
    }

    //  for loops

    // rust for loops are exclusive of the last element of the list
    for z in 0..5 {
        // this prints from 0 to 4
        println!("the value of z is: {}", z);
    }

    // another way of doing the same thing but the last element is inclusive

    for z in 0..=5 {
        println!("the value of z is: {}", z);
    }

    // iterating over an array
    let arr = [100, 200, 300];

    for z in arr {
        println!("the value of z is: {}", z);
    }
}
