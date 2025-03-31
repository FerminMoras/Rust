use std::io::stdin;

fn main() {
    let mut int: u32 = 10;
    let mut dato = String::new();

    println!("Ingrese un numero entero sin signo");
    stdin().read_line(&mut dato).expect("error al leer dato");

    let int_user: u32 = dato.trim().parse().expect("valor incorrecto");

    int = int + int_user;

    println!("El resultado es: {}", int * int);
}
