// Struct Example 
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
// Struct without variable names example
#[derive(Debug)]
struct Rec(u32, u32);

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn checker() -> bool {
        true
    }
}

fn area_rec(rectangle: &Rec) -> u32{
    rectangle.1 * rectangle.0 
}



fn main() {
    let rec1 = Rectangle{
        width: 20,
        height: 10,
    };
    println!("The area of the first Rectangle {rec1:#?} is: {}", rec1.area());

    let rec2 = Rec(32,2);
    println!("Area of the second Rectangle {rec2:#?} is: {}", area_rec(&rec2));
    Rectangle::checker();
}
