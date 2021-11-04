//use std::fmt;

fn main() {
   //comment test

   /*
   multi_line

   stejny jak v influx_1
   */

   let x = 2 * 2 * 2 ; //3
   let y = 10;
   let z = "foookin_paavel";
   println!("2 * 2 * 2 = {}", x);

   println!("deset: {deset}\ntext: {text}", text=z, deset=y);

   //println!("hello_world");
   //println!("I'm a Rustacean!");

   let int = 66;
   println!("\n     76543210           76543210");
   println!("int: {int:0>width$} -> bin:   {bin:0>width$}", int=int, width=8, bin=format!("{:b}", int));

   println!("{:>width$}\n", format!("{:#010b}", 66), width=22+8+2);

   #[allow(dead_code)]
   struct Structure(i32);

   //println!("This struct `{}` won't print...", Structure(3));


   let pi = 3.141592;
   println!("pi: {pi:.3} / not_rounded: {pi}", pi=pi);


   /* 2>stderr
   eprintln!("bad boy");
   */
   }
   