use super::*;

#[test]
fn test_run() {
    let config = Config {
        query: "the".to_string(),
        filename: "poem.txt".to_string(),
    };
    assert_eq!(run(config).is_ok(), true);
}

#[test]
fn test_run_error() {
    let config = Config {
        query: "the".to_string(),
        filename: "poem1.txt".to_string(),
    };
    assert_eq!(run(config).is_err(), true);
}

#[test]
fn test_search() {
    let query = "duct";
    let contents = "Rust:\nsafe, fast, productive.\nPick three.";
    assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
}

#[test]
fn test_search_null() {
    let query = "null";
    let contents = "Rust:\nsafe, fast, productive.\nPick three.";
    assert_eq!(search(query, contents), Vec::<&str>::new());
}