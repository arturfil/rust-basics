pub fn ownership_ie() {
    let s = "hello";
    let mut t = String::from(s);
    t.push_str(", world");
    println!("{t}");
}
