pub fn remplazar_pares(v:&mut[i32;3]) {
    for i in 0..3 {
        if v[i] % 2 == 0 {
            v[i] = -1;
        }
    }
}

#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn test_remplazar_pares(){
        let mut vec = [2,5,6];
        remplazar_pares(&mut vec);
        assert_eq!(vec,[-1,5,-1]);
    }
}    