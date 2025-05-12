pub fn multiplicar_valores(v: &mut[i32;3], factor: i32) {
    for i in 0..3 {
        v[i] *= factor;
    }
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_multiplicar_valores(){
        let mut vector= [3,6,9]; 
        multiplicar_valores(&mut vector, 3);
        assert_eq!(vector, [9,18,27]);
    }
}    