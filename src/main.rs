use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    // rust 中的泛型函数指定类型的方式 (turbofish)
    // func::<type> 与 c++ 中的 func<type> 类似
    // let args_turbofish = env::args().collect::<Vec<String>>();
    // dbg!(args_turbofish);

    // 可以简单使用 _ 占位符代替具体的 String 类型，但是 Vec<_> 不能省略，因为还可能是 HashSet<_> 等其他类型
    let args: Vec<_> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
