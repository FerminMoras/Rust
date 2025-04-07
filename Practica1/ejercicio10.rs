fn main() {
    let array1 = [1, 1, 1, 1, 1];
    let array2 = [1, 1, 1, 1, 1];
    let mut array3 = [0; 5];

    for i in 0..5 {
        array3[i] = array1[i] + array2[i];
    }

    for elemento in array3 {
        println!("{}", { elemento });
    }
}
