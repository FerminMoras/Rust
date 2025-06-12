trait es_primo {
    fn es_primo(&self) -> bool;
}
impl es_primo for i32 {
    fn es_primo(&self) -> bool {
        if *self <= 1 {
        return false;
    }

    if *self == 2 {
        return true;
    }

    if *self % 2 == 0 {
        return false;
    }

    else {
        let raiz = (*self as f64).sqrt() as i32;

            for i in 3..=raiz {
                if *self % i == 0 {
                    return false;
                }
            }
            return true;
        }
    }
    }       

fn contar_primos(v: Vec<i32>) -> usize {
    v.iter().filter(|&n| n.es_primo()).count()
}

#[test]
fn test_es_primo() {
    let v = vec![3,5,7,13,21,25,29];
    let cant = contar_primos(v);
    assert_eq!(cant,5);
}