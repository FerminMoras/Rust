pub fn ordenar_nombres (v: &mut [String;3]) {
    v.sort();
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_ordenar_nombres() {
        let mut vec = ["hola".to_string(),"chau".to_string(),"andres".to_string()];
        ordenar_nombres(&mut vec);
        assert_eq!(vec,["andres".to_string(),"chau".to_string(),"hola".to_string()])
    }
}