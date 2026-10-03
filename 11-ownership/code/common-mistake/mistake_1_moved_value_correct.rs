fn print(message: String){
  println!("{}", message);
}

fn main(){
   let message = String::from("It's not okay!");
   print(message.clone());
   println!("{}", message);
}