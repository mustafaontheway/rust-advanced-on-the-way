fn main() {
    
}

#[derive(Debug)]
struct Player<T> {

    name: String,
    score: T
}

impl<T> Player<T> 
where 
    T: std::ops::AddAssign + std::ops::SubAssign
{

    fn new(name: String, score: T) -> Self {

        Player { name, score }
    }

    fn inc_score(&mut self, inc: T) {

        self.score += inc;
    }

    fn dec_score(&mut self, dec: T) {

        self.score -= dec;
    }
}

