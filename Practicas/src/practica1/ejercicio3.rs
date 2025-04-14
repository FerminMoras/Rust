use std::io::stdin;

fn main() {
    let mut ok_and: bool = false;
    let mut ok_or: bool = false;
    let mut dato = String::new();

    println!("ingrese un booleano");
    stdin().read_line(&mut dato).expect("Error al leer el dato");
    let booleano: bool = dato.trim().parse().expect("dato incorrecto");

    ok_and = ok_and & booleano;
    ok_or = ok_or | booleano;

    println!("Resultado usando la operacion AND: {}", ok_and);
    println!("Resultado usando la operacion OR: {}", ok_or);
}
