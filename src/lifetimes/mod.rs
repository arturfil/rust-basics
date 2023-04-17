// fn longest(x: &str, y: &str) -> &str { // this would be wrong
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn lifetimes_main() {
   let r;
   /*
    * This code will error out because x doesn't live long enough.
    {
       let x = 5;
       r = &x;
    }
   */

    let str1 = String::from("abcdasdfasde");
    let str2 = "&str";
    println!("r: {}", r);

    let result = longest(str1.as_str(), str2);
    println!("The longest string is {}", result);
}
