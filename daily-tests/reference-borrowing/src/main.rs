fn main() {
    let mut s = String::from("Hello, world");
    append_exclamation(&mut s);
    println!("{} legnth : {}", s, longest_word(&s));
}

fn longest_word(s: &str) -> usize {
    s.len()
}

fn append_exclamation(s: &mut String) {
    s.push_str("!");
}
