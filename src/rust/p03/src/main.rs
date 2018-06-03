fn main() {
    let s = Some(String::from("abc"));
    let mut s0 = &mut s;

    s0 = match s0.as_ref() {
        Some(sopt) => {
        },
        None => Some(String::from("xyz"))
    };
    // borrow(&mut s);
    // s = String::from("cde");
}


fn borrow(s: &mut String) {
    print!("{}", s)
}
