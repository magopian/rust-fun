use std::io;

struct Rectangle {
    width: usize,
    height: usize,
}

fn main() {
    println!("Enter the width of the rectangle:");
    let mut width = String::new();
    io::stdin().read_line(&mut width).expect(
        "Failed to read line",
    );
    let width = width.trim().parse().expect("That's not a number!");

    println!("Enter the height of the rectangle:");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect(
        "Failed to read line",
    );
    let height = height.trim().parse().expect("That's not a number!");

    println!("The area is: {}", area(Rectangle { width, height }));
}

fn area(rect: Rectangle) -> usize {
    rect.width * rect.height
}
