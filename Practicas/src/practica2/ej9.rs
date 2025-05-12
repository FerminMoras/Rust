pub fn cantidad_en_rango(v: [i32;3], inf: i32, sup:i32) -> i32 {
    let mut cant = 0;

    for elemento in v {
        if elemento >= inf && elemento <= sup {
            cant += 1;
        }  
    }

    return cant;
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_cantidad_en_rango(){
        let resultado = cantidad_en_rango([12,14,7], 10, 20);
        assert_eq!(resultado,2);
    }
}    