fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);


    //let number_in_hex: u32 = 0x80; 
    let number_in_hex: u32 = 128;

    println!("\n                        87654321                                87654321");
    println!("int: {nh} hex: 0x{nh:x} bin: {nh:08.b } >> 2 is int: {shift} hex: 0x{shift:x} bin: {shift:08.b}", nh=number_in_hex, shift = number_in_hex >> 2);
    //println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);


    //let number: u32 = 1042;
    let number: u32 = 66;

    println!("\n                     87654321         6543210987654321");
    println!("number: {n:03}, binary: {:08.b} << 8 is {:016.b}", n=number, b=number << 8);


}
