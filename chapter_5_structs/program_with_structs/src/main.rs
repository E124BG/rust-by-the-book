struct Rectangle {//def of the struct with 2 fields
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {//creation of one instance of struct
        width: 30,
        height: 50,
    };

    println!(
        "The calculate_area_rectangle of the rectangle is {} square pixels.",
        calculate_area_rectangle(&rect1)
    );
}

fn calculate_area_rectangle(rectangle: &Rectangle) -> u32 {//function to calculate calculate_area_rectangle of a rectangle (takes a ref to a rectangle as attribute)
    rectangle.width * rectangle.height
}
