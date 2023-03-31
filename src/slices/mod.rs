
pub fn first_words(s: &String) -> usize {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
          return i;
       }
   }
   s.len()
}

pub fn imprvd_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

pub fn slices_main() {
    let s = String::from("Hello world");

    let hello_world = &s[..];
    let hello = &s[..5];
    let world = &s[6..];

    println!("1.{} 2.{}", hello, world);
    println!("1.{} ", hello_world);

    let first = imprvd_first_word(&s);
    println!("First word is: {first}");

    // slices could be of any type

    let a = [1,2,3,4,5];
    let slice = &a[..3];

    assert_eq!(slice, &[1,2,3]);
}


