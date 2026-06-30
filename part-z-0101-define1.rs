fn main() {


}

trait Book {

    fn short_summary(&self) -> String;

    fn set_info(&mut self, name: String, writer: String, year: u16);
}
