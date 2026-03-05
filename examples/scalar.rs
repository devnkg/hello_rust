#![allow(unused)]


/*
---------------------------------------------------------------------------------
RUST SCALAR TYPES - QUICK REFERENCE TABLE
---------------------------------------------------------------------------------
TYPE      | BITS | RANGE (Approx)          | REAL WORLD USE CASE
---------------------------------------------------------------------------------
INTEGERS (Ginti)
u8        | 8    | 0 to 255                | Colors (RGB), Age, Small Counters
i8        | 8    | -128 to 127             | Temperature (Celsius)
u16       | 16   | 0 to 65,535             | Port Numbers, Small Game Scores
i16       | 16   | -32,768 to 32,767       | Small Bank Balance/Debt
u32       | 32   | 0 to 4.2 Billion        | YouTube Views, Large Counters
i32       | 32   | -2.1B to 2.1B           | DEFAULT, General Logic, Coordinates
u64       | 64   | 0 to 18 Quintillion     | File Sizes, Timestamps, DB IDs
i64       | 64   | -9Q to 9Q               | Scientific Data, Global Wealth
u128/i128 | 128  | Infinite-ish            | Cryptography, Blockchain, Physics
usize     | Arch | 32-bit or 64-bit        | Array Indexing, Memory Size
isize     | Arch | 32-bit or 64-bit        | Memory Offsets (Rarely used)

FLOATS (Decimal wale)
f32       | 32   | Single Precision        | Simple Decimals (Petrol Price)
f64       | 64   | Double Precision        | DEFAULT, Scientific Precision

OTHERS
bool      | 8* | true / false            | Switches, Flags (Light ON/OFF)
char      | 32   | Any Unicode (A, अ, 🦀)  | Single Letters, Emojis, Symbols
---------------------------------------------------------------------------------
*Note: bool takes 1 byte (8 bits) for performance reasons.
---------------------------------------------------------------------------------
*/


fn main() {
    // --- Signed Integers ---
    let temperature: i8 = -20;
    let loan: i16 = -15000;
    let population: i32 = 1200000;
    let stars_in_galaxy: i64 = 100_000_000_000; // Underscore (_) readability ke liye hai
    let national_debt: i128 = 2000000000000000000;

    println!("--- Signed Integers ---");
    println!("Temperature: {}°C", temperature);
    println!("Bank Loan: {}", loan);
    println!("Population: {}", population);
    println!("Stars in Galaxy: {}", stars_in_galaxy);
    println!("National Debt: ${}\n", national_debt);

    // --- Unsigned Integers ---
    let age: u8 = 30;
    let bank_balance: u16 = 50000;
    let crypto_value: u128 = 340282366920938463463374607431768211455;

    println!("--- Unsigned Integers ---");
    println!("Age: {}", age);
    println!("Bank Balance: {}", bank_balance);
    println!("Crypto Extreme Value: {}\n", crypto_value);

    // --- Boolean & Character ---
    let is_raining: bool = true;
    let grade: char = 'A';
    let emoji: char = '😊';

    println!("--- Boolean & Char ---");
    println!("Is it raining? {}", is_raining);
    println!("Grade: {}", grade);
    println!("Emoji of the day: {}\n", emoji);

    // --- Floating-point numbers ---
    let petrol_price: f32 = 3.59;
    let pi: f64 = 3.141592653589793;

    println!("--- Floating Points ---");
    println!("Petrol Price: {}", petrol_price);
    println!("Value of PI: {}\n", pi);

    // --- Architecture dependent ---
    let array_index: usize = 5;
    let memory_offset: isize = -10;

    println!("--- Architecture Dependent ---");
    println!("Array Index (usize): {}", array_index);
    println!("Memory Offset (isize): {}\n", memory_offset);

    // --- Type Conversion & Overflow Check ---
    let big_number: i32 = 300;
    let small_number = big_number as u8; // Truncation (300 - 256 = 44)

    let my_int: i32 = 65; // 65 'A' ka ASCII code hai
    let my_char = my_int as u8 as char;

    println!("--- Type Conversion ---");
    println!("Original i32: {}, Converted to u8 (Truncated): {}", big_number, small_number);
    println!("Integer 65 converted to Char: {}", my_char);
    println!("Is 10 equal to 10? {}", eq(10, 10));
    println!("final value of 154.13 + 120.34 + 130.45: {}", add_floats(154.13, 120.34 + 130.45));
    println!("Casting u8, i8, f32 to f32: {}", cast(10, 15, 18.9))
}

pub fn eq(a: i32, b: i32) -> bool {
    a == b
}

pub fn add_floats(x: f32, y: f32) -> f32 {
    x + y
}

pub fn cast(x: u8, y:i8, z:f32) -> f32 {
    x as f32 + y as f32 + z as f32
}