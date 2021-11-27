fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { ref name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);

    println!("name: {n} age: {a}", n=person.name, a=person.age);

    //let ref ref_person = person;
    let mut ref_person = person;
    println!("The person struct is {:?}", ref_person);
    ref_person.age += 13;
    println!("The person struct is {:?}", ref_person);
    println!("ref_: {n} age: {a}", n=ref_person.name, a=ref_person.age);
}

