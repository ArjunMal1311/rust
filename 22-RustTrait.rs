// Shared functionality for multiple types

trait MyTrait {
    // method signatures
    fn method_one(&self);
    fn method_two(&mut self, arg: i32) -> bool;
}

struct MyStruct {
    value: i32,
}

impl MyTrait for MyStruct {
    // implementation of method_one
    fn method_one(&self) {
        println!("The value is: {}", self.value);
    }

    // implementation of method_two
    fn method_two(&mut self, arg: i32) -> bool {
        if arg > 0 {
            self.value += arg;
            return true;
        } else {
            return false;
        }
    }
}