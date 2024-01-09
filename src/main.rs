// a cli tool for showcasing echo 
fn main() {
   let a : Vec<String> = std::env::args().collect();
    a.iter().for_each(|i| print!("{}",i));
}
