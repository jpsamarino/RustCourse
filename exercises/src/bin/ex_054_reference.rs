
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s.clone();
    let r2 = &mut s;

   r1.push_str("string");
    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
}