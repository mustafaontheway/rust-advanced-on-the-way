fn main() {


}

trait Novel {

    fn short_summary(&self) -> String;

    fn set_info(&mut self, name: String, novelist: String, year: u16);
}
