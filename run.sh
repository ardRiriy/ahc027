#!/bin/bash

# 開始番号と終了番号を設定
start=0
end=5

# 並列実行するプロセスの数
num_procs=4

# 初期化
rm -rf out
rm -rf ahc027_tool/out
rm -rf ahc027_tool/res
mkdir out
mkdir ahc027_tool/res

# 指定された範囲の数値に対してループ
seq -f "%04g" $start $end | xargs -P $num_procs -I {} sh -c 'cargo run --bin ahc027-a < in/{}.txt > out/{}.txt'

# 出力を移動
mv out ahc027_tool

cd ahc027_tool

seq -f "%04g" $start $end | xargs -P $num_procs -I {} sh -c 'cargo run -r --bin vis in/{}.txt out/{}.txt > res/{}.txt'

