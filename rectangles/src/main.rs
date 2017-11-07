use std::io;

struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
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
    let rect1 = Rectangle { width, height };

    println!("The area is: {}", rect1.area());


    println!("Enter the width of a second rectangle:");
    let mut width2 = String::new();
    io::stdin().read_line(&mut width2).expect(
        "Failed to read line",
    );
    let width2 = width2.trim().parse().expect("That's not a number!");

    println!("Enter the height of a second rectangle:");
    let mut height2 = String::new();
    io::stdin().read_line(&mut height2).expect(
        "Failed to read line",
    );
    let height2 = height2.trim().parse().expect("That's not a number!");
    let rect2 = Rectangle {
        width: width2,
        height: height2,
    };

    println!(
        "The second rectangle fits in the first one: {}",
        rect1.can_hold(&rect2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 2,
            height: 2,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 2,
            height: 2,
        };
        assert!(!smaller.can_hold(&larger));
    }
}
