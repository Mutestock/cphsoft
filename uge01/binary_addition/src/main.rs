mod gates;
mod circuits;


trait SomeTrait {}

fn main() {
    let some_vec: Vec<Box<dyn SomeTrait>> = Vec::new();
}
