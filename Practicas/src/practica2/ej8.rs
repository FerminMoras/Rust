pub fn sumar_arreglos(v1: [f64;3],v2: [f64;3]) -> [f64;3] {
    let mut v3 = [0.0;3];

    for i in 0..3 {
        v3[i] = v1[i] + v2[i];
    }

    return v3;
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_sumar_arreglos(){
        let resultado = sumar_arreglos([20.0,10.0,5.0],[20.0,10.0,5.0]);
        assert_eq!(resultado,[40.0,20.0,10.0]);
    }
}