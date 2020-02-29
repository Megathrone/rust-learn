use std::ops::Mul;
use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: i32 = v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v2 = vec![1, 2, 3, 4, 5];

    match v2.get(1) {
        Some(i) => println!("{}", i),
        None => println!("Wrong"),
        _ => {}
    }

    let mut v3 = vec![1, 2, 3, 4, 5, 6];
    for i in &mut v3 {
        *i = i.mul(10);
    }

    for i in &mut v3 {
        println!("{}", i);
    }

    let data = "hello word";

    let mut s = data.to_string();
    s.push_str("123");
    let b = s.as_str();

    println!("{} + {}", s, b);

    let name = String::from("孙哥");
    let nation = String::from("Japan");

    let mut map = HashMap::new();
    map.insert(name, nation);
    println!("{}",map.get(&name));

}
