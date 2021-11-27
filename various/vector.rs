fn main() {
    //let mut v = Vec::new();
    let mut v = vec![1, 2, 3, 4];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("vector: {:?}\nfirst: {}\nget: {:?}",
             v,
             &v[0],
             &v.get(10));

    for vvv in &mut v {
        *vvv += 10;
        println!("{}", vvv);
    }

    println!("first: {}",
             v[0]);
}
