#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn find_area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rectangle = Rectangle{
        height: 30,
        width: 50
    };

    let scale = 2;
    let _rectangle2 = Rectangle {
        height: 44,
        width: dbg!(30 * scale) // This works because dpg! returns ownership after logging the output.
    };

    println!("{:#?}", rectangle);
    let area1 = rectangle.find_area();
    println!("The area of the rectangle is {} square pixels.", area1);
}

