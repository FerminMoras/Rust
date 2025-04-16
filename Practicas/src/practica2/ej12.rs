pub fn remplazar_pares(v:&mut[i32;3]) {
    for i in 0..3 {
        if v[i] % 2 == 0 {
            v[i] = -1;
        }
    }
}