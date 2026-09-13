enum Color {
    Red,
    Green,
    Blue,
}

fn red_handler() {
    println!("Red color selected");
}

fn green_handler() {
    println!("Green color selected");
}

fn blue_handler() {
    println!("Blue color selected");
}

fn main() {
    let colors = vec![Color::Blue, Color::Green, Color::Red];

    for color in colors {
        match color {
            Color::Red => red_handler(),
            Color::Green => green_handler(),
            Color::Blue => blue_handler(),
        }
    }
}
