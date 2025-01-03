fn main() {
    // let mut name = String::new();
    let mut s1: String = String::from("hello");
    let s2 = &mut s1;
    let s3 = s2;

    do_something(s2);
    println!("This is the s1 {}", s2);
}

fn do_something(s2: &mut String) {
    // println!("This is the s2 {}", s2);
    // return s2;
    s2.push_str(" world");

    println!("{}", s2);
}


