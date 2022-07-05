
#[allow(unused_mut)]


fn main() {
    
    let square = |x| x * x;
    println!("5 squared is {}", square(5));

    let pairs = vec![(0, 1), (2, 3), (4, 5)];
     pairs
         .into_iter()
         //.map(|t| (t.0 + 1, t.1))
         .map(|(x, y)| (x + 1, y))
         .for_each(|t| println!("{:?}", t));

    
    let mut numbers = vec![1, 2, 3, 4];
    for x in &mut numbers {
        *x *= 3; // multiply the value by 3 via the mutable reference x
    }
    println!("{:?}", numbers); // should print [3, 6, 9, 12]
    
    
    let words = vec!["autobot", "beach", "car", "decepticon", "energon", "frothy"];
    let transformed = words.into_iter()
    .filter(|s| !s.contains("h"))
    .map(|w| w.to_uppercase())
    .collect::<Vec<_>>(); // do the stuff here
    println!("Transformed: {:?}", transformed);

    
}
