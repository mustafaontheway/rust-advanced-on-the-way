fn main() {

    let _point1: Points<u8> = Points::z(17);

    let  _point2: Points<f32> = Points::y(4.345);
}

enum Points<T> {

    x(T), y(T), z(T)
}
