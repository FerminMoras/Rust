fn main() {
    let tupla: (&str, [i32; 5]) = ("Hola gente", [2, 4, 6, 8, 10]);
    let mut suma = 0;

    for i in 0..5 {
        suma += tupla.1[i];
    }

    println!("{}", tupla.0);
    println!("la suma total de los elementos del arreglo es: {}", suma);
}
