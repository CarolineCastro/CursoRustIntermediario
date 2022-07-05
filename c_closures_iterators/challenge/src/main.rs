#[allow(unused_mut)]


fn main() {
    
    let mut pairs = vec![(0, 1), (2, 3), (4, 5)];
    for pair in &mut pairs {
        pair.0 += 1;
        println!("{:?}", pair);
    }
   
    let mut numbers = vec![1, 2, 3, 4];
    numbers
        .into_iter()
        .map(|n| n * 3)
        .for_each(|n| println!("{}", n));
    
}
