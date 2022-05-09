use clap::{ArgEnum, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "bb",
    author = "Ruman Suyama",
    version = "0.1.0",
    about = "野球の打者指標を算出する"
)]
struct Options {
    #[clap(short, long, value_name = "STATS", arg_enum, required = false, help = "昇順に並べ替える")]
    order:Stats,

    #[clap(short, long, help = "降順に並べ替える")]
    descending:bool,

    #[clap(value_name = "FILE", required = true, help = "打者の基本成績がまとめられたcsvファイル")]
    file:PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Stats {
    K,
    BB,
    AVG,
    SLG,
    OBP,
    OPS,
}

fn main() {
    Options::parse();
}

fn hello(name: Option<String>) -> String {
    return format!("Hello, {}", if let Some(n) = name {
        n
    } else {
        "World".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!("Hello, World", hello(None));
        assert_eq!("Hello, Suyama", hello(Some("Suyama".to_string())));
    }
}
