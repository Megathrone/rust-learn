fn main() {
    let _a = build_person(String::from("nmsl"), 18);
    let _b  = User {
        name : String::from("孙哥"),
        .._a
    };
    println!("{}", _a.name);
    println!("{}", _b.name);
}

struct User {
    name: String,
    age: i32,
}

fn build_person(name: String, age: i32) -> User {
    User {
        name,
        age,
    }
}