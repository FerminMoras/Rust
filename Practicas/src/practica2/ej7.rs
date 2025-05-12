pub fn cantidad_mayores (vector: [i32;3], num: i32) -> i32 {
    let mut cant = 0;
    for elemento in vector {
        if elemento > num {
            cant += 1;
        }
    }

    return cant;
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_cantidad_mayores(){
        let resultado = cantidad_mayores([15,3,17], 5);
        assert_eq!(resultado,2);
    }
}