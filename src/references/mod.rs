

pub fn references_n_borrowing() {
    let s1 = String::from("hello");
    calculate_length(&s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
