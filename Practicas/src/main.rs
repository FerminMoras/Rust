mod practica2;

fn main() {

    println!("Ejercicio 1");

    let num = 31;
    
    if practica2::ej1::es_par(num) {
        println!("El numero es par");
    }
    else {
        println!("El numero es impar");
    }

    let num2 = 12;

    println!("Ejercicio 2");

    if practica2::ej2::es_primo(num2) {
        println!("El numero es primo");
    }
    else {
        println!("El numero no es primo");
    }

    println!("Ejercicio 3");

    let vector = [21,20,23,20,27];
    println!("la suma total de los elementos del vector es: {}", practica2::ej3::suma_pares(vector));

    println!("Ejercicio 4");
    println!("la cantidad de elementos impares es: {}", practica2::ej4::cantidad_impares(vector));

    println!("Ejercicio 5");
    let vector_duplicado = [15.5,23.5,10.4,11.1,32.3];
    println!("vector duplicado {:?}", practica2::ej5::duplicar_valores(vector_duplicado));

    println!("Ejercicio 6");
    
}
