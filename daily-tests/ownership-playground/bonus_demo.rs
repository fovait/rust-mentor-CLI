// Bonus: chain ownership through functions with Vec<i32>

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", numbers);

    // Each function takes ownership, transforms, and returns it.
    // The same name `numbers` shadows the previous owner — clean pipeline.
    let numbers = double_all(numbers);
    let numbers = keep_even(numbers);
    let numbers = add_ten(numbers);

    println!("Final:   {:?}", numbers);
    println!("Count:   {}", count(numbers)); // ownership ends here — dropped
}

// Takes ownership of v, transforms it, returns ownership to caller
fn double_all(v: Vec<i32>) -> Vec<i32> {
    v.iter().map(|n| n * 2).collect()
    // v is dropped at end of scope, but we've already extracted what we need
}

fn keep_even(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().filter(|n| n % 2 == 0).collect()
    // into_iter() takes ownership of v's elements (no & needed)
}

fn add_ten(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().map(|n| n + 10).collect()
}

fn count(v: Vec<i32>) -> usize {
    v.len() // v is dropped when this function ends
}
