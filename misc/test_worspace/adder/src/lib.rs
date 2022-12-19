#[cfg(feature="feature1")]
pub fn add(left: usize, right: usize) -> usize {
    println!("feature 1");
    left + right
}

#[cfg(feature="feature2")]
pub fn add(left: usize, right: usize) -> usize {
    println!("feature 1");
    left + right
}
