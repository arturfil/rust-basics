
pub fn vectors_main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1,2,3];

    v.push(5);
    v.push(6);

   for num in &v {
        println!("num: {}", num); 
   }

   let third: &i32 = &v[2];
   println!("Third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("Third element is {third}"),
        None => println!("No 3rd element was found"),
    }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);
    let first = &v[0];

    // v.push(9); would give error because with a new value added, references 
    // may have to change.
    // println!("first element {first}");
    
    let vec2 = vec![0,40,30];
    for i in &vec2 {
        println!("{i}");
    }

    let mut sqrs = vec![1,2,3,4,5];
    for x in &mut sqrs {
        let before = &x;
        *x *= 2;
    } 

    for k in &sqrs {
       println!("no. {k}"); 
    }
}
