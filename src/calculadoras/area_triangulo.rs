pub fn area_triangulo(base: f64, height: f64)-> f64{
    let result=do_calculate(base, height);
    return result;
}

fn do_calculate(base: f64, height: f64)-> f64{
    return (base * height)/2.0;
}