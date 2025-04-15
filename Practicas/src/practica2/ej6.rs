pub fn longitud_de_cadenas(vector: [&str; 5]) -> [usize; 5] {
    let mut vec_enteros = [0; 5];

    for i in 0..5 {
        vec_enteros[i] = vector[i].chars().count(); //.count devuelve un entero referenciando la longitud del string.
    }

    return vec_enteros;
}
