fn main() {

    let _v1 = dummy(4);
    let _v2 = dummy(2.29);
    let _v3 = dummy(Dummy::Dummy1);
    let _v4 = dummy("lorem");
    let _v5 = dummy("lorem".to_string());
    let _v6 = dummy(Dummy::Dummy2);

    let _v7 = dummy::<f32>(2.29); // turbofish eperator
    let _v8: u8 = dummy(6); // i like this way
}

enum Dummy {
    Dummy1,
    Dummy2
}

fn dummy<T>(dummy_val: T) -> T {

    dummy_val
}
