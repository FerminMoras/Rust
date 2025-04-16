pub fn cantidad_cadenas_mayor_a (v:[String;3], lim:usize) -> i32 {
    let mut cant = 0;
    for i in 0..3 {

        if v[i].chars().count() > lim {
            cant += 1;
        }
    }
    return cant;
}