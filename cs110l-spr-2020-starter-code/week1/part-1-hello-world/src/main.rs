fn main() {
    println!("Goodbye, world!");
    let mut n = 1;
    n = n + 1;
    println!("Number is {}", n);

    let mut s = String::from("Hello "); // "String" type annotation is optional
    s.push_str("world!");
    println!("{}", s);

    let mut v = Vec::new();
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    let mut arr: [i32; 4] = [0, 2, 4, 8];
    arr[0] = -2;
    println!("{}", arr[0] + arr[1]);

    for i in v.iter() { // v is the vector from above
        println!("{}", i);
    }
}
