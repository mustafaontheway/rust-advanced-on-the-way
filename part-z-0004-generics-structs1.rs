fn main() {

    let _c1: Coordinates<u8> = Coordinates { x: 11, y: 4, z: 5 };

    let _c2: Coordinates<f32> = Coordinates { x: 2.13, y: -4.34, z: 8.99 };
}

struct Coordinates<T> {

    x: T,
    y: T,
    z: T
}
