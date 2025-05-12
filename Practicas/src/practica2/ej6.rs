pub fn longitud_de_cadenas(vector: [&str; 5]) -> [usize; 5] {
    let mut vec_enteros = [0; 5];

    for i in 0..5 {
        vec_enteros[i] = vector[i].chars().count(); //.count devuelve un entero referenciando la longitud del string.
    }

    return vec_enteros;
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_longitud_de_cadenas(){
        let resultado = longitud_de_cadenas(["uno","dos","tres","cuatro","cinco"]);
        assert_eq!(resultado,[3,3,4,6,5]);
    }
}
