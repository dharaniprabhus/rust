enum Vehicle{
    Unknown,
    Car{desc: String},
    Motorbike{desc: String},
}

fn main(){
    let car = Vehicle::Car{desc: String::from("I have a petrol engine")};
    let motorbike = Vehicle::Motorbike{desc: String::from("I have two wheels")};
    let vehicles = vec![car,motorbike,Vehicle::Unknown];
    for v in vehicles{
        match v{
            Vehicle::Car{desc} => println!("{}",desc),
            Vehicle::Motorbike{desc} => println!("{}",desc),
            Vehicle::Unknown => ()
        }
    }
}

