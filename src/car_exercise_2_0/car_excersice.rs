#[derive(PartialEq, Debug)]
pub struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}
#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_quality(mile: u32) -> (Age, u32) {
    let quality: (Age, u32) = if mile == 0 {
        (Age::New, mile)
    } else {
        (Age::Used, mile)
    };
    return quality;
}

fn car_factory(color: String, transmission: Transmission, convertible: bool, miles: u32) -> Car {

    if car_quality(miles).0 == Age::Used {
        if convertible {
            println!("Preparing a used car: {:?}, {}, Hard top, {} miles", transmission, color, miles);
        } else {
            println!("Preparing a used car: {:?}, {}, Convertible, {} miles", transmission, color, miles);
        }
    } else {
        if convertible {
            println!("Building a new car: {:?}, {}, Hard top, {} miles", transmission, color, miles);
        } else {
            println!("Building a new car: {:?}, {}, Convertible, {} miles", transmission, color, miles);
        }
    }
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        age: car_quality(miles),
    }
}

pub fn make_car() {
    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut engine = Transmission::Manual;
    let mut car: Car;

    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.convertible, car.transmission, car.color, car.age.1
    );

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.convertible, car.transmission, car.color, car.age.1
    );

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.convertible, car.transmission, car.color, car.age.1
    );
}
