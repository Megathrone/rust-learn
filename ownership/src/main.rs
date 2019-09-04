fn main() {
    let s1: String = String::from("hello world");
    let ret = cal_length(&s1);
    println!("{}", ret);
    let mut s2: String = String::from("nmsl");
    change(&mut s2);
    println!("{}", s2);
    let s3 = "孙 哥";
    println!("{}", first_word(&s1));
    println!("{}",first_word(s3));
}

fn cal_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("adasd");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &num) in bytes.iter().enumerate() {
        if num == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
