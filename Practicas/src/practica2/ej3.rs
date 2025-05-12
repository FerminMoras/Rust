
pub fn suma_pares(vector: [i32;5]) -> i32 {
    let mut suma = 0;

    for elemento in vector {
        suma += elemento;
    }

    return suma;
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_suma_pares() {
        let vector = [10,10,10,10,10];
        let resultado = suma_pares(vector);
        assert_eq!(resultado,50);
    }
}        