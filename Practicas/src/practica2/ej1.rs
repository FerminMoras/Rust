
pub fn es_par (num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_par_retorne_true() {
        let resultado = es_par(10);
        assert_eq!(resultado, true);
    }

    #[test]
    fn test_es_par_retorne_false() {
        let resultado = es_par(11);
        assert_eq!(resultado, false);
    }
}