# bb

[![build](https://github.com/rmnsym/bb/actions/workflows/build.yaml/badge.svg)](https://github.com/rmnsym/bb/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/rmnsym/bb/badge.svg?branch=main)](https://coveralls.io/github/rmnsym/bb?branch=main)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/rmnsym/bb)](https://rust-reportcard.xuri.me/report/github.com/rmnsym/bb)

野球の打者指標を算出する
![baseball_logo](sport_baseball_bat.png)

## Description
このソフトウェアでは野球の打者の打席数や安打数などの基本成績から打率やOPSなどの指標を算出する．

## Usage
```sh
bb [OPTIONS] <FILE>

ARGUMENTS
    <FILE>    打者の基本成績がまとめられたcsvファイル

OPTIONS
    -d, --descending-order    降順に並べ替える
    -h, --help                Print help information
    -s, --sort <STATS>        昇順に並べ替える [possible values: kpct, bbpct, avg, slg, obp, ops]
    -V, --version             Print version information
```

## Sample Output
```sh
$ bb hitting.csv                     
No.,Player,K%,BB%,AVG,SLG,OBP,OPS
1,京産次郎,17.6,10.1,.243,.408,.336,.744
2,神山太郎,17.5,8.3,.217,.425,.292,.716
3,産大三郎,20.2,16.0,.283,.485,.395,.880
$ bb -s　"ops" hitting.csv
No.,Player,K%,BB%,AVG,SLG,OBP,OPS
1,神山太郎,17.5,8.3,.217,.425,.292,.716
2,京産次郎,17.6,10.1,.243,.408,.336,.744
3,産大三郎,20.2,16.0,.283,.485,.395,.880
$ bb -s　"ops" -d hitting.csv
No.,Player,K%,BB%,AVG,SLG,OBP,OPS
1,産大三郎,20.2,16.0,.283,.485,.395,.880
2,京産次郎,17.6,10.1,.243,.408,.336,.744
3,神山太郎,17.5,8.3,.217,.425,.292,.716
```
