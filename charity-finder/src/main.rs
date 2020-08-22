extern crate clap;
use clap::{Arg, App, SubCommand};
use text_io::read;

fn main() {
    println!("Which issue matters the most to you?");
    println!("1. Criminal Justice Reform");
    println!("2. Climate Change");
    println!("3. Healthcare");
    let choice: i32 = read!();
    println!("You chose {}!", choice);
}
