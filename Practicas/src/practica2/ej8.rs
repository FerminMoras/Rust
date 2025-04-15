pub fn sumar_arreglos(v1: [f64;3],v2: [f64;3]) -> [f64;3] {
    let mut v3 = [0.0;3];

    for i in 0..3 {
        v3[i] = v1[i] + v2[i];
    }

    return v3;
}