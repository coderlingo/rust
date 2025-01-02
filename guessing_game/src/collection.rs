fn main() {
    let vec: Vec<i32> = Vec::new();
    let mut vec1 = vec![1,2,3];
    vec1.push(4);
    let val = vec1.get(5);
    match val {
        None => println!("Nothing found"),
        Some(val) => println!("Value is {}", val),
    }
    let s = "Здравствуйте";
    println!("the value of s : {} for length {}", &s[0..2], s.len());
}