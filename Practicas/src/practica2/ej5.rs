pub fn duplicar_valores(vector: [f32;5]) -> [f32;5] {
    let mut duplicado: [f32;5] = [0.0,0.0,0.0,0.0,0.0];

    for i in 0..5 {
        duplicado[i] = vector[i]*2.0;
    }

    return duplicado;
}