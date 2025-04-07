fn main() {
    const VALOR: i32 = 2;
    let mut array = [2, 4, 6, 8, 10, 12];

    for mut elemento in array.iter_mut() {
        *elemento *= VALOR;
        println!("valor {elemento}");
    }
}
