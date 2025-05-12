pub fn cantidad_cadenas_mayor_a (v:[String;3], lim:usize) -> i32 {
    let mut cant = 0;
    for i in 0..3 {

        if v[i].chars().count() > lim {
            cant += 1;
        }
    }
    return cant;
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_cantidad_cadenas_mayor_a(){
        let vec: [String;3] = ["boca".to_string(),"hola".to_string(),"chau".to_string()]; 
        let resultado = cantidad_cadenas_mayor_a(vec, 3);
        assert_eq!(resultado,3);
    }
}