pub fn perimetro_cuadrado(lado:f64)->f64{
    let result=do_calculate(lado);
    return result;
}

fn do_calculate(lado:f64)->f64{
    return lado*4.0;
}