use std::io::stdin;

fn main() {
    let array = ["hola", "anonimo", "como", "estas?", "chau"];
    let mut dato = String::new();
    let mut i = 0;
    let mut ok = false;

    println!("Ingrese una cadena de texto a buscar");
    stdin()
        .read_line(&mut dato)
        .expect("error al leer la cadena");
    let cadena = dato.trim();

    while i < 5 && !ok {
        if array[i] == cadena {
            println!("la cadena se encuentra en el arreglo");
            ok = true;
        } else {
            i += 1;
        }
    }

    if ok == false {
        println!("la cadena no se encuentra en el arreglo");
    }
}
