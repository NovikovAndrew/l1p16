fn main() {
    let sorted_vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let target = 7;

    match sorted_vec.binary_search(&target) {
        Ok(index) => println!("Элемент найден на позиции: {}", index),
        Err(_) => println!("Элемент не найден"),
    }
}