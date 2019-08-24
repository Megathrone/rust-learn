fn main() {
    let s1: String = String::from("hello world");
    let ret = cal_length(&s1);
    println!("{}", ret);
}

fn cal_length(s: &String) -> usize {
    s.len()
}
