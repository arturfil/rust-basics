

pub fn strings_main() {
    let s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents";
    let t = String::from("initial content");
    

    let mut k = String::from("foo");
    k.push_str("bar");

    let mut lol = String::from("lo");
    lol.push('l');

    println!("{k}");
    println!("{lol}");

    let s1 = String::from("Hello,");
    let s2 = String::from("world!");
    let s4 = String::from("What is this!");
    // let s3 = s1 + &s2;

    // println!("{s3}");

    let s5 = format!("{s1} {s2} {s4}");
    println!("{s5}");

    for ch in "Arturo".chars() {
        println!("{ch}");
    }

    for bt in "Arturo".bytes() {
        println!("{bt}");
    }

}
