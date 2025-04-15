pub fn cantidad_en_rango(v: [i32;3], inf: i32, sup:i32) -> i32 {
    let mut cant = 0;

    for elemento in v {
        if elemento >= inf && elemento <= sup {
            cant += 1;
        }  
    }

    return cant;
}