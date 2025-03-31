use std::io::stdin;

fn main() {
    let x: f32 = 10.5;
    let mut num = String::new();

    println!("Ingrese un numero real");
    stdin().read_line(&mut num).expect("Error al leer");
    let numero: f32 = num.trim().parse().expect("Ingrese un numero valido");

    let suma = x + numero;
    let multiplicacion = x * numero;
    let resta = x - numero;
    let division = x / numero;

    println!("Suma: {}", suma);
    println!("Resta: {}", resta);
    println!("Multiplicacion: {}", multiplicacion);
    println!("Division: {}", division);
}
