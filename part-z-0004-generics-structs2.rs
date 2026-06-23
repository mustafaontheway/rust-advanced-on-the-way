fn main() {

    let d1: Data<i16, &str> = Data { data1: -5000, data2: 321, data3: "Loss... No profit!" };

    println!("Sales quantity: {} - Loss amount: ${} - final words: '{}'", d1.data2, d1.data1, d1.data3)
}

#[derive(Debug)]
struct Data<T, U> {

    data1: T,
    data2: T,
    data3: U
}

// Sales quantity: 321 - Loss amount: $-5000 - final words: 'Loss... No profit!'
