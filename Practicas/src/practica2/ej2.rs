pub fn es_primo(num: i32) -> bool {
    if num <= 1 {
        return false;
    }

    if num == 2 {
        return true;
    }

    if num % 2 == 0 {
        return false;
    }

    else {
        let raiz = (num as f64).sqrt() as i32;

            for i in 3..=raiz {
                if num % i == 0 {
                    return false;
                }
            }
            return true;
        } 
}