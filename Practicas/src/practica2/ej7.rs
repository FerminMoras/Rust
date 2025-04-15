pub fn cantidad_mayores (vector: [i32;3], num: i32) -> i32 {
    let mut cant = 0;
    for elemento in vector {
        if elemento > num {
            cant += 1;
        }
    }

    return cant;
}