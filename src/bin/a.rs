/*
このコード、と～おれ!
  ∧＿∧　
（｡･ω･｡)つ━☆・*。
⊂    ノ       ・゜+.
 しーＪ       °。+ *´¨)
                  .· ´¸.·*´¨) ¸.·*¨)
                                      (¸.·´ (¸.·'* ☆
*/

// -*- coding:utf-8-unix -*-
use ac_library::*;
use itertools::Itertools;
use proconio::input;


fn solve(){
    input! {
        n: usize,
        wall_h: [[usize; n]; n - 1],
        wall_v: [[usize; n]; n - 1],
        d: [[usize; n]; n],
    }
}

fn main() {
    let mut i: usize = 1;

/*    /* 複数テストケースならコメントアウトを外す */
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    i = input.trim().parse().unwrap();*/

    while i != 0 {
        solve();
        i -= 1;
    }
}


