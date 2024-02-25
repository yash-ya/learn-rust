fn main_main() {
    let s = String::from("Hello");
    take_ownership(s);
    let m: i32 = 4;
    make_copy(m);
    println!("{}", m);

    let s1 = String::from("YASH");
    let len = calculate_length(s1);
    println!("{}", len);
}

fn calculate_length(s: String) -> usize {
    s.len()
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn make_copy(m: i32) {
    println!("{}", m);
}
