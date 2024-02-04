fn main() {
    // mutating variables
    let mut x = 5;
    println!("value of x is : {} ", x);
    x = 6;
    println!("value of x is : {}", x);


    // this is a constant
    const THIS_IS_A_CONST: u32 = 60*60;


    // shadowing 
    let y= 5;

    let y = y +2;

    println!("value of y is : {}", y);

    {
        let y = y * 2;
        
        println!("value of y with in the scope is : {}", y); // 14
    }

    println!("value of y is : {}", y); //7

    
  let mut z: u32 = 1;
  {
    let mut z = z;
    // this mutate the newly created z varible. not the initial z 
   // The statement z += 2 only affects the shadowed z inside the inner curly braces, 
   //not the outer z
    z += 2;
    println!("{z}"); // 3
  }
  println!("{z}"); // 1


  //  Floating points

  let f64_number = 4.585;
  let f32_number: f32 = 4.558;


}
