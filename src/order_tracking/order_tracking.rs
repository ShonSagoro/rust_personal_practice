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

    let mut color=order as usize;

    if color>4{
        color=color-4;
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
  let mut order = 1;
  let mut car: Car;

  car = car_factory(order, 1000);
  orders.insert(order, car);
  println!("Car order {}: {:?}", order, orders.get(&order));

  // Car order #2: Used, Convertible
  order = order + 1;
  car = car_factory(order, 2000);
  orders.insert(order, car);
  println!("Car order {}: {:?}", order, orders.get(&order));    

  // Car order #3: New, Hard top
  order = order + 1;
  car = car_factory(order, 0);
  orders.insert(order, car);
  println!("Car order {}: {:?}", order, orders.get(&order));

  // Car order #4: New, Convertible
  order = order + 1;
  car = car_factory(order, 0);
  orders.insert(order, car);
  println!("Car order {}: {:?}", order, orders.get(&order));

  // Car order #5: Used, Hard top
  order = order + 1;
  car = car_factory(order, 3000);
  orders.insert(order, car);
  println!("Car order {}: {:?}", order, orders.get(&order));

  // Car order #6: Used, Hard top
  order = order + 1;
  car = car_factory(order, 4000);
  orders.insert(order, car);
  println!("Car order {}: {:?}", order, orders.get(&order));
    

}
