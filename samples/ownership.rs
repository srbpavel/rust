/*fn takes_ownership(some_string: String) {
   println!("own: {}", some_string);
}


fn makes_copy(some_integer: i32) {
   println!("copy: {}", some_integer);
}*/


fn calculate_length(s: &String) -> usize { // type of param in REF
   s.len()
}


fn change(some_string: &mut String) { // REF is also MUT
   some_string.push_str(", is kInG");
}


//fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {

   let s = s.trim(); // shadow + TRIM for whitespace
   
   let bytes = s.as_bytes();
   for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
       	  return &s[0..i];
       }
   }
   &s[..] // if does not find ' ' return full slice
}


fn main() {
   #![allow(unused_variables)]

   let s1 = String::from("hello");
   let len = calculate_length(&s1); // REF
   println!("The length of '{}' is {}.", s1, len);


   let mut p = String::from("fookume"); // MUT
   change(&mut p); // REF + MUT -> only ONCE !!!


   println!("mut_p: {}", p);

   let len = p.len();

   println!("f: {}", &p[..=3]); // [0..=3] [0..4] 
   println!("k: {}", &p[len-4..len]); // [12..]
   println!("full_cream: '{}'", &p[..]);


   let sss = String::from("   zdar_a_silu najdes v syru ");
   let sss_literal = "   zdar_a_silu najdes v syru ";

   println!("fn slice: >{}< / original: '{}'", first_word(&sss), sss);
   println!("fn slice_literal: >{}< / original: '{}'", first_word(&sss_literal[..]), sss_literal);

   let a = [1, 2, 3, 4, 5];
   let b = &a[0..3];
   
   println!("array: {:?} / {:?}", b, a);

   #[derive(Debug)] //DEBUG {#:?} for dict_print
   struct Dict {
   	  name: String,
	  stall: String,
	  age: u8,
	  active: bool,
   }

   let mut kun = Dict {
       	       name: String::from("Wonka"),
	       stall: String::from("krenek"),
	       age: 9,
	       active: true};

   println!("semik >> {:#?}", kun);

   kun.name = String::from("willy_wonka"); // UPDATE
   println!("name: {}", kun.name);


   /* 
   let s1 = String::from("foookin");
   takes_ownership(s1);
   //println!("error: {}", s1);

   let x = 5;
   makes_copy(x);
   println!("valid: {}", x);

   // let s2 = s1.clone();

   //println!("{}, paavel", s1);
   // println!("s1 = {}, s2 = {}", s1, s2);
   */


   /*
   let x = 5;
   let y = x; // NO CLONE for u32 / bool / f64 / char / tuple(u32, bool, f64, cahr)
   
   println!("x = {}, y = {}", x, y);
   */


   /*{
	//let s = "hello"; //LITERAL cannot be mutated
   	let mut s = String::from("foookin"); // STRING

   	s.push_str(", paavel");

   	println!("s: {}", s);

   }*/
   

}