/**
 * !!! 二进制和库 crate 包的最佳实践
 *
 * 一个包 package 可以同时包含一个 src/main.rs 二进制 crate 根
 * 和一个 src/lib.rs 库 crate 根，并且这两个 crate 默认以包名来命名。
 *
 * 通常，这种包含二进制 crate 和库 crate 的模式的包，
 * 在二进制 crate 中只保留足以生成一个可执行文件的代码，并由可执行文件调用库 crate 的代码。
 *
 * 模块树应该定义在 src/lib.rs 中。
 * 这样通过以包名开头的路径，公有项就可以在二进制 crate 中使用。
 * 二进制 crate 就变得同其它在该 crate 之外的、使用库 crate 的用户一样：
 * 二者都只能使用公有 API。这有助于你设计一个好的 API；你不仅仅是作者，也是用户！
 */

#[cfg(test)]
mod tests;

mod config;
use std::fs;
use std::error::Error;
pub use crate::config::Config;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &content) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}