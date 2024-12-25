use std::fs;
use std::env;

fn main() {
    // rust 中的泛型函数指定类型的方式 (turbofish)
    // func::<type> 与 c++ 中的 func<type> 类似
    // let args_turbofish = env::args().collect::<Vec<String>>();
    // dbg!(args_turbofish);

    let args_iter: env::Args = env::args();
    dbg!(&args_iter);
    let args: Vec<String> = args_iter.collect();
    dbg!(&args);
    println!("args = {args:?}");
    println!("args = {args:#?}");

    let query: &String = &args[1];
    let filename: &String = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
}
