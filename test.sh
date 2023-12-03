#!/bin/bash

# 開始番号と終了番号を設定
start=0
end=99

# 並列実行するプロセスの数
num_procs=5

rm -rf out
mkdir out

# 指定された範囲の数値に対してループ
seq -f "%04g" $start $end | xargs -P $num_procs -I {} sh -c 'cargo run --bin ahc027-a < in/{}.txt > out/{}.txt'