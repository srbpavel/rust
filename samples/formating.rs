use std::fmt; // Import `fmt`


#[derive(Debug)]
struct Age {
    value: i32,
}


impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "(DISPLAY AGE: {})", self.value)
    }
}


impl fmt::Binary for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bin = format!("DISPLAY BINARY: {:b}", self.value);
	f.pad_integral(true, "", &bin)      
    }
}

fn main() {

   let name = "foookin paavel";

   println!("default_display: {}", name);
   println!("debug: {:?}", name);

   let age = Age {value: 350};
   //let int = 13;

   println!("default_display: {}", age);
   println!("debug: {:?}", age);
   println!("binary: {:b}", age);

}
