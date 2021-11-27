use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[\n")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ",\n")?; }
            write!(f, "{i}: {l}", i=count, l=v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "\n]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3, 4, 5]);
    println!("{}", v);


    let sss = 1..8;

    //let ref ref_sss = sss; // borrow LEFT
    let ref_sss = &sss; // borrow RIGHT
    println!("REF: {:?}", ref_sss); //REF musim volat driv nez puvodni ???

    for (count, r) in sss.into_iter().enumerate() 
    	{ 
    	  println!("{} -> {}", count, r);
	}



}
