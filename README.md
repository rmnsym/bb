# bb
野球の打者指標を算出する
![baseball_logo](sport_baseball_bat.png)

## Description
このソフトウェアでは野球の打者の打席数や安打数などの基本成績から打率やOPSなどの指標を算出する．

## Usage
```sh
bb [OPTIONS] <FILE>
OPTIONS
    -a, --ascending-order <STATS>   昇順に並べ替える
    -d, --descending-order <STATS>  降順に並べ替える
    -h, --help                      このメッセージを出力する
ARGUMENTS
    FILE                            打者の基本成績がまとめられたcsvファイル
```

## Sample Output
```sh
$ bb hitting.csv                     
No.,Player,K%,BB%,AVG,SLG,OBP,OPS
1,京産次郎,17.6,10.1,.243,.408,.336,.744
2,神山太郎,17.5,8.3,.217,.425,.292,.716
3,産大三郎,20.2,16.0,.283,.485,.395,.880
$ bb -a　"OPS" hitting.csv
No.,Player,K%,BB%,AVG,SLG,OBP,OPS
1,神山太郎,17.5,8.3,.217,.425,.292,.716
2,京産次郎,17.6,10.1,.243,.408,.336,.744
3,産大三郎,20.2,16.0,.283,.485,.395,.880
$ bb -d　"OPS" hitting.csv
No.,Player,K%,BB%,AVG,SLG,OBP,OPS
1,産大三郎,20.2,16.0,.283,.485,.395,.880
2,京産次郎,17.6,10.1,.243,.408,.336,.744
3,神山太郎,17.5,8.3,.217,.425,.292,.716
```
