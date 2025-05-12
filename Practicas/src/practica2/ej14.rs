pub fn incrementar(f: &mut f64) {
    *f += 1.0;
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    
    fn test_incrementar(){
        let mut num = 2.0;
        incrementar(&mut num);
        assert_eq!(num,3.0);
    }
}    