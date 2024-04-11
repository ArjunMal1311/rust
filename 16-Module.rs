mod config {
    // items in modules by default have private visibility
    fn select() {
        println!("called config::select");
    }

    // use the `pub` keyword to override private visibility
    pub fn print() {
        println!("called config::print");
    }
}

fn main() {
    // private items inside module cannot be accessed outside the parent module
    // calling private select function inside config module will cause a compilation error
    display::select();
}