pub fn multiplicar_valores(v: &mut[i32;3], factor: i32) {
    for i in 0..3 {
        v[i] *= factor;
    }
}