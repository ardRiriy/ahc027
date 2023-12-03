#!/bin/bash

# 開始番号と終了番号を設定
start=0
end=999

# 並列実行するプロセスの数
num_procs=5

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

# 総和を格納する変数
total=0

# 0000 から 0999 までループ
for i in $(seq -f "%04g" 0 999); do
    # ファイル名を生成
    filename="res/${i}.txt"

    # ファイルが存在する場合のみ処理を実行
    if [ -f "$filename" ]; then
        # ファイルから数値を抽出し、総和に加算
        score=$(grep -oP 'Score = \K\d+' "$filename")
        total=$((total + score))
    fi
done

# 総和を表示
echo "Total score: $total"