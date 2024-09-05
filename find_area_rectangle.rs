fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

This program is nice and running well, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related. It would be more readable and more manageable to group width and height together.

So, the better code is:

fn main () {
    let rectangle = (30,50);

    println!("The area of the rectangle is {} square pixels", area(rectangle));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


But we can make it even more clear:

struct Rectangle {
    width: u32,
    height: u32,
}

fn main () {
    let react1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels", area(&react1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}




Another kind of code:

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}
