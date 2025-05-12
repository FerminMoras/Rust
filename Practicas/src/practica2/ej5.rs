pub fn duplicar_valores(vector: [f32;5]) -> [f32;5] {
    let mut duplicado: [f32;5] = [0.0,0.0,0.0,0.0,0.0];

    for i in 0..5 {
        duplicado[i] = vector[i]*2.0;
    }

    return duplicado;
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_duplicar_valores() {
        let resultado = duplicar_valores([2.0,2.0,2.0,2.0,2.0]);
        assert_eq!(resultado, [4.0,4.0,4.0,4.0,4.0]);
    }
}