use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]

pub struct Options {
    // #[clap(short, long, value_name = "STATS", arg_enum, required = false, help = "昇順に並べ替える")]
    // pub sort: Stats,
    //
    // #[clap(short, long, help = "降順に並べ替える")]
    // pub descending_order: bool,

    #[clap(value_name = "FILE", required = true, help = "打者の基本成績がまとめられたcsvファイル")]
    pub file: PathBuf,
}

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
// pub enum Stats {
//    KPCT,
//    BBPCT,
//    AVG,
//    SLG,
//    OBP,
//    OPS,
//}
