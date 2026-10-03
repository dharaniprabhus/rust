enum Vehicle {
    Unknown,
    Car { desc: String, color: String },
    Motorbike { desc: String },
}

fn main() {
    let car = Vehicle::Car {
        desc: String::from("I have a petrol engine"),
        color: String::from("red"),
    };
    let motorbike = Vehicle::Motorbike {
        desc: String::from("I have two wheels"),
    };
    let vehicles = vec![car, motorbike, Vehicle::Unknown];
    for v in vehicles {
        match v {
            Vehicle::Car { desc, color } => println!("{} color is {}", desc, color),
            Vehicle::Motorbike { desc } => println!("{}", desc),
            Vehicle::Unknown => (),
        }
    }
}
