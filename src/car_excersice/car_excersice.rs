pub struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic
}

fn car_factory(color: String, transmission: Transmission, convertible: bool)-> Car {

    // Use the values of the input arguments
    // All new cars always have zero mileage
    let car = Car{
        color: color, transmission: transmission, convertible:convertible, mileage:0
    };

    return car;
}

pub fn make_car(){
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
    //puedes darle formato a valores como los enums usando {:?}.

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:#?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
    //el {:#/} en esencia es lo mismo, solo que añade sangrias, donde lo necesite para que sea mas legible.

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:#?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);    

    //esto formatos {:?} y {:#?} sirven igual que el toString en java, es como que saca los atributos y les da una forma en que se pueda leer, esta chido.
}