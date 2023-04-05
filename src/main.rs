use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let s1 = Rc::new(RefCell::new(String::from("hello")));
    let s2 = s1;

    // This line will not compile because s1 has been moved to s2
    println!("s1 = {}", s1.borrow());

    // This line will compile because s2 is still in scope
    println!("s2 = {}", s2.borrow());

    let mut s3 = String::from("hello");
    let len = calculate_length(&s3);
    println!("The length of '{}' is {}.", s3, len);

    change(&mut s3);
    println!("s3 = {}", s3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
