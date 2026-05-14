fn main() {
    let hi = String::from("Hello World!");
    let hello = hi.clone();
    
    println!("clone from hi : {}", hello);
    let get_hi = print_hi(hi);
    
    println!("get value from 'print_hi' : {}", get_hi);
}

fn print_hi(hi: String) -> String {
    println!("{}", hi);
    hi
}
