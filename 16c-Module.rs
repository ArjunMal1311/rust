pub mod player {
    pub mod sprite {
        pub fn create() {
            println!("Hello worldllld!");
        }
    }
}


use player::sprite::create;

fn main(){
    create();
}