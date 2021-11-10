use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}


// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    //x: f64,
    //y: f64,
    x: i32,
    y: i32,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}


#[derive(Debug)]
struct Complex {
    rrr: f64,
    iii: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} + {}i)", self.rrr, self.iii)
    }
}


//BINARY
// https://doc.rust-lang.org/std/fmt/
// umim formatovat ale musel jsem pretypovat na int
// spravne: 10.75
// https://www.log2base2.com/number-system/float-to-binary-conversion.html
// http://sandbox.mc.edu/%7Ebennet/cs110/flt/dtof.html
// INTEGRAL:
//   10 = (1010)2
// FRACTIONAL:
//  0.75 * 2 =>1.50 // 1 take 0.50
//  0.50 * 2 =>1.00 // 1 stop
//  0.75 = (11)2
//   back: (1/2 + ((1/2)/2)) = (0.5 + 0.25) = 0.75
//
//   (01)2
//   back: (0/2 + ((1/2)/2)) = 0 + 0.25 = 0.25

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bin = format!("x: {:b} y: {:b}", self.x, self.y);
	//let bin_y = format!("{:b}", self.x);
	f.pad_integral(true, "", &bin)      
    }
}


fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    //let point = Point2D { x: 3.3, y: 7.2 };
    let point = Point2D { x: 66, y: 213 };

    let complex = Complex { rrr: 123.456, iii: 78.90 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    //println!("What does Point2D look like in binary: {:b} ?", point);
    println!("Binary: {:b} ?", point);



    println!("complex_Display: {}", complex);
    println!("complex_Debug: {:?}", complex);

}
