
pub fn cantidad_impares(vector: [i32;5]) -> i32 {
    let mut cant = 0;

    for elemento in vector {
        if elemento % 2 != 0 {
            cant += 1;
        }
    }

    return cant;
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]    
    fn test_cantidad_impares() {
        let resultado = cantidad_impares([11,13,5,4,2]);
        assert_eq!(resultado,3);
    }
 }
