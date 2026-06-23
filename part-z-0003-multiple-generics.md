fn main() {

    let _v1 = dummy("lorem", "lorem ipsum".to_string());

    let (_v2, _) = dummy(3.14, -50);

    let (_v4, _v5): (u8, u16) = dummy(99, 2026);
}

fn dummy<T, U>(dummy_val1: T, dummy_val2: U) -> (T, U) {

    (dummy_val1, dummy_val2)
}
