fn main() {
    let array = [2, 4, 6, 8, 10];
    let mut suma = 0;

    for elemento in array {
        suma += elemento;
    }

    println!("La suma total de los elementos del arreglo es {}", suma);
}
