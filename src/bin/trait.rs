trait Area {
    fn show(&self);
}

struct Circle {
    r: f32,
}

struct Square {
    l: f32,
}

impl Area for Circle {
    fn show(&self) {
        println!("Circle area is {}", self.r * self.r * 3.14);
    }
}

impl Area for Square {
    fn show(&self) {
        println!("Square area is {}", self.l * self.l);
    }
}

fn main() {
    let c = Circle { r: 5.0 };
    let s = Square { l: 4.0 };

    let areas: Vec<Box<dyn Area>> = vec![Box::new(c), Box::new(s)];
    for area in areas {
        area.show();
    }
}
