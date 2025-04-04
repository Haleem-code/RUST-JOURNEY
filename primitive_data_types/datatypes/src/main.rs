//RUst has Signed and Unsigned integers
// Signed integers can be positive or negative
// Unsigned integers can only be positive
// Signed integers are represented with a "i" prefix
// Unsigned integers are represented with a "u" prefix
// The size of the integer is represented with a number after the prefix
// For example, i8 is a signed 8-bit integer, and u8 is an unsigned 8-bit integer
// The size of the integer can be 8, 16, 32, 64, or 128
// The size of the integer can also be "size" which is the size of the pointer
fn main() {
    let x:i8=-4;
    let y:u32=5;
    println!("Signed Integer:{}", x);
    println!("Unsigned Integer:{}", y);


        //FLOATS [Floating Point Types]
        // Floating point types are used to represent decimal numbers
        // Floating point types are represented with a "f" prefix
        // The size of the floating point type is represented with a number after the prefix
        // For example, f32 is a 32-bit floating point type, and f64 is a 64-bit floating point type
        // The size of the floating point type can be 32 or 64
        // The size of the floating point type can also be "size" which is the size of the pointer  
            let pi:f64=3.14;
            println!("Value of pi is:{}", pi);


            //BOOLEANS [Boolean Types]
            let haleem_willbe_a_good_rust_programmer:bool=true;
            println!("Haleem will be a good Rust programmer:{}", haleem_willbe_a_good_rust_programmer);

            let is_rust_fun:bool=true;
            println!("Is Rust fun? {}", is_rust_fun);


            //CHARACTER [Character Types]
            // Character types are used to represent a single character
    let letter:char='a';
    println!("The letter is: {}", letter);
    
        }


