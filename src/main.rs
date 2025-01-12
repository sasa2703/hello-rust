use ferris_says::say;
use std::io::{stdout,BufWriter};
use std::io;

fn main(){
    let stdout = stdout();
    let message = String::from("Hello fellow Teodor!");
    let width = message.chars().count();


    let mut writer = BufWriter::new(stdout.lock());
    say(&message,width,&mut writer).unwrap();
    let x =plus_one(3);
    println!("Pluse one equals: {x}");
    fun_name();

    for number in (1..4).rev(){
        println!("N:{number}")
    }
    

    let b = [10, 20, 30, 40, 50];

    for element in b {
        println!("the value is: {element}");
    }
}
    
    fn plus_one(x:i32) -> i32 {
        x+1
    }

    fn fun_name() {
    let a = [1,2,3,4,5];
    println!("Please enter an array index.");
    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize =index.trim().parse().expect("Index entered was not a number");
    
    let element = a[index];
    println!("The value of the element at index {index} is :{element}");
    }
