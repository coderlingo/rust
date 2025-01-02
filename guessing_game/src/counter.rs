fn main() {
    let mut counter = 0;
    let mut direction = 1;
    loop {
        println!("Value of the counter is {}", counter);
        counter = counter + direction; 
        if counter == 0 {break;}
        if counter == 9 {direction = -1;}
    }
    println!("End count: {}", counter)


}