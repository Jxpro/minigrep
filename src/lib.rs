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

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    // 我目前认为：函数返回值与其中涉及到函数参数，需要标注一样的生命周期，这条规则我认为编译器可以根据函数内部代码进行推断。
    // 所以简单理解手动标注的目的，是为了方便编译器可以在只知道函数签名的情况下，对函数参数和返回值外部的生命周期进行检查。
    // 并且我暂时没有发现不可判定的代码案例，即它们除生命周期标注不同以外，其余函数体内容完全相同，且均可编译通过。

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    // 注意无法之前如下复用 search 函数，因为 to_lowercase 会返回一个新的 String，存在生命周期问题
    // search(&query.to_lowercase(), &contents.to_lowercase())

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
