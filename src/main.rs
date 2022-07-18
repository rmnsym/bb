// use clap::{ArgEnum, Parser};
use clap::Parser;
use std::path::PathBuf;
use std::fs;

mod cli;

fn round_num(num: f32) -> f32 {
    return (num * 1000.0).round();
}

fn change_type(data: &str) -> f32 {
    return data.parse().unwrap();
}

fn read_file(path_buf: PathBuf) -> String {
    return fs::read_to_string(path_buf).unwrap();
}

fn main() {
    let opts = cli::Options::parse();
    let path_buf = opts.file;
    let csvtext = read_file(path_buf);
    let str2: &str = &csvtext;
    let mut count = 0;
    println!("No.,Player,K%,BB%,AVG,SLG,OBP,OPS");
    for line in str2.lines() {
        if count > 0 {
            let data :Vec<&str> = line.split(',').collect();
            let ab: f32 = change_type(data[2]);
            let pa: f32 = change_type(data[3]);
            let h: f32 = change_type(data[4]);
            let single: f32 = change_type(data[5]);
            let double: f32 = change_type(data[6]);
            let triple: f32 = change_type(data[7]);
            let hr: f32 = change_type(data[8]);
            let so: f32 = change_type(data[9]);
            let bb: f32 = change_type(data[10]);
            let hbp: f32 = change_type(data[11]);
            let sac_fly: f32 = change_type(data[12]);
            let kpct = round_num(so / pa) / 10.0;
            let bbpct = round_num(bb / pa) / 10.0;
            let avg = round_num(h / ab) / 1000.0;
            let slg = round_num((single + double * 2.0 + triple * 3.0 + hr * 4.0) / ab) / 1000.0;
            let obp = round_num((h + bb + hbp) / (ab + bb + hbp + sac_fly)) / 1000.0;
            let ops = round_num(obp + slg) / 1000.0;
            println!("{},{},{},{},{},{},{},{}", data[0], data[1], kpct, bbpct, avg, slg, obp, ops);
        }
        count = count + 1;
    }
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
    #[test]
    fn test_read_file() {
        let path_buf = PathBuf::from("./hitting.csv");
        assert_eq!("No.,Player,AB,PA,H,1B,2B,3B,HR,SO,BB,HBP,Sac Fly\r\n1,京産次郎,604,698,188,110,29,1,48,110,86,6,2\r\n2,神山太郎,478,546,135,62,31,0,42,153,62,2,4\r\n3,産大三郎,502,654,157,106,20,2,29,93,145,2,5",readfile(path_buf));
    }
    #[test]
    fn test_round_num() {
        let pre_num: f32 = 1.111;
        let num: f32 = 1111;
        assert_eq!(num, round_num(pre_num));
    }
    #[test]
    fn test_change_type() {
        let pre_data: &str = "100";
        let data: f32 = 100;
        assert_eq!(data, change_type(pre_data))
    }
}
