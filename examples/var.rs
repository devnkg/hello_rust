#![allow(unused)]


//Constant
    const NUM: u32 = 12;

fn main() {
    //variables
    // - immutable by default
    // - use mut keyword to make it mutable

    let mut x = 1; 


    // Type inference
    let y: i32 = -1;
    let z = -1;

    // Shadowing
    let x: i32 = 1;
    let x: i32 = 4;
    let x: bool = true;

    // Type Placeholder
    let x: _ = 1234;

    // println!
    let x = 1;
    println!("x is {}", x);
    // Inline, 
    println!("x is {x}");
    //Positional arguments
    println!("{0} + {0} = {1}", x, x + x);
    // Debug
    println!("DEBUG: x {:?}", x);
    println!("DEBUG: x {:#?}", x)

}