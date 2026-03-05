#! [allow(unused)]

fn main() {
    let mut x = u32::MAX;
    x += 1; // This will cause an overflow
        
    println!("u32 max: {}, x: {}", u32::MAX, x);
    
    // u32:: checked_add() method se overflow ko handle kar sakte hain
    let x = u32::checked_add(3, 1);
    println!("Checked add result: {:?}", x); // This will return None due to overflow
    // u32:: wrapping_add() method se overflow ko wrap around kar sakte hain


    
}