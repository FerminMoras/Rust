
pub fn cantidad_impares(vector: [i32;5]) -> i32 {
    let mut cant = 0;

    for elemento in vector {
        if elemento % 2 != 0 {
            cant += 1;
        }
    }

    return cant;
}