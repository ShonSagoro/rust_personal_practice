use std::collections::HashMap;

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
    if mile > 0 {
        return (Age::Used, mile);
    }
    (Age::New, mile)
}

fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut color = order as usize;
    while color > 4 {
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color - 4;
    }

    let mut transmission= Transmission::Manual;
    let mut convertible= true;

    if order%3==0{
        transmission= Transmission::Automatic;
    }else if order%2==0{
        transmission=Transmission::SemiAuto;
        convertible=false;
    }

    Car {
        color: String::from(colors[(color-1) as usize]),
        transmission: transmission,
        convertible: convertible,
        age: car_quality(miles),
    }
}

pub fn make_car() {
  let mut orders: HashMap<i32, Car> = HashMap::new();
  let mut car: Car;
  let mut miles = 0;

  for order in 1.. 12{
    car = car_factory(order, miles);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
    if miles == 2100 {
        miles = 0;
    } else {
        miles = miles + 700;
    }
  }

 



}
