fn main() {

    let mut novel_mahser = Novel::new("Mahşer".to_string(), "Peyami Safa".to_string(), 1924);

    novel_mahser.consider_this(); // default implementation

    // novel_mahser.set_info();

    novel_mahser.novelist = "P. Safa".to_string();

    // novel_mahser.set_info();

    // let summary = novel_mahser.short_summary("Lorem ipsum lorem ipsum lorem ipsum".to_string());

    // println!("Novel short summary: {}", summary);
}

trait Book {

    fn short_summary(&self, summary: String) -> String;

    fn set_info(&mut self);

    fn consider_this(&self) {

        println!("Reading is a good habit...")
    }
}

struct Novel {

    novel_name: String,
    novelist: String,
    pub_year: u16,
}

impl Novel {
    
    fn new(novel_name: String, novelist: String, pub_year: u16) -> Self {
        
        Self { novel_name, novelist, pub_year }
    }
}

impl Book for Novel {
    
    fn short_summary(&self, summary: String) -> String {
        
        summary
    }

    fn set_info(&mut self) {
        
        println!("Novel Name: {} - Novelist: {} - year: {}", self.novel_name, self.novelist, self.pub_year)
    }
}

// Novel Name: Mahşer - Novelist: Peyami Safa - year: 1924
// Novel Name: Mahşer - Novelist: P. Safa - year: 1924
// Novel short summary: Lorem ipsum lorem ipsum lorem ipsum
