fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);


    let range = 1..10;
    println!("range: {:?}", range.into_iter().enumerate());

    //let copy_range = range;
    //println!("copy_range: {:?}", copy_range.into_iter().enumerate());
}

