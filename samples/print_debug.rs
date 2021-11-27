#[derive(Debug)]
struct DebugPrintable(i32);


#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);


#[derive(Debug)]
struct Person<'a> {
    full_name: &'a str,
    age: u8
}


fn main() {

   println!("{:?}", DebugPrintable(123));


   println!("ik ben {1:?} aka {0:?} -> ik kom uit {actor:?}",
            "foookume",
            "paavel",
            actor="tjechie");

   // `Structure` is printable!
   println!("Now {:?} will print!", Structure(3));
    
   // The problem with `derive` is there is no control over how
   // the results look. What if I want this to just show a `7`?
   println!("Now {:?} will print!", Deep(Structure(7)));


   let full_name = "srb pavel";
   let age = 42; //2021-1979;
   let pavel = Person { full_name, age };

   // Pretty print
   println!("{:#?}", pavel);

   //ERROR {}
   //OK
   println!("{:?}", pavel); //STRUCT

   println!("items: {name} -> {age}", name=pavel.full_name, age=pavel.age); //ITEMS

}
