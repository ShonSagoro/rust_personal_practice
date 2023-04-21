mod calculadoras;
use calculadoras::area_triangulo::area_triangulo;
use calculadoras::circunferencia::circunferencia;
use calculadoras::perimetro_cuadrado::perimetro_cuadrado;

fn main() {
    println!("el resultado del area es de: {}", area_triangulo(15.0,16.0));
    println!("el resultado de la circunferencia es de: {}", circunferencia(15.0));
    println!("el resultado del del perimetro es de: {}", perimetro_cuadrado(15.0));

}
