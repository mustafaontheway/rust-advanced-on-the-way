fn main() {

    let _player1: Player<u8> = Player::new("Ayhan Bilir".to_string(), 87);

    let _player2: Player<f32> = Player::new("Bilge Büyük".to_string(), 77.23);
    
}

#[derive(Debug)]
struct Player<T> {

    name: String,
    score: T
}

impl<T> Player<T> {

    fn new(name: String, score: T) -> Self {

        Player { name, score }
    }
}

