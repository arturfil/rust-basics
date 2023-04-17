use std::fmt::Display;

// fn longest(x: &str, y: &str) -> &str { // this would be wrong
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_2(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

struct ImportantExcerpt<'a> {
    part: &'a str, 
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// If there is a self reference, no need to add lifetime
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// static lifetime
// let s: &'static str = "I have static lifetime";

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where 
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lifetime elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

pub fn lifetimes_main() {
   /*
    * This code will error out because x doesn't live long enough.
    {
       let x = 5;
       r = &x;
    }
   */

    let str1 = String::from("abcdasdfasde");
    let result;
    {
        let str2 = String::from("&str");

        result = longest(str1.as_str(), str2.as_str());

    }
    println!("The longest string is {}", result);

    let i = ImportantExcerpt {
        part: &String::from("first part"),
    };
}
