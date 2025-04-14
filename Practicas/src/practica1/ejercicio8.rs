use std::io::stdin;

fn main() {
    const CADENA: &str = "son todos fallen";
    let mut dato = String::new();

    println!("Ingrese un caracter a buscar en la cadena");
    stdin().read_line(&mut dato).expect("error al leer dato");

    let caracter = dato.trim().chars().next().expect("caracter invalido");

    let cant = CADENA.chars().filter(|&c| c == caracter).count();

    println!("el caracter '{}' se repite: {}", caracter, cant);
}
