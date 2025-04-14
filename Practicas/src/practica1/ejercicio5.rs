use std::io::stdin;

fn main() {
    let mut cadena: String = "Hola, mi nombre es".to_string();
    let mut dato = String::new();

    println!("Ingrese la cadena a concatenar");
    stdin().read_line(&mut dato).expect("Error al leer el dato");

    cadena += &(" ".to_owned() + &dato);

    println!("{}", cadena.to_uppercase());
}
