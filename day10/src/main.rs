use pipe::maze::*;


pub mod pipe;

fn main() {
    let maze = create_map("test.data"); 
    println!("{:?}", maze);
}
