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
use proconio::input;
use proconio::marker::Chars;


fn dfs(i: usize, j: usize, n: usize ,visited: &mut Vec<Vec<bool>>, wall_h: &Vec<Vec<char>>, wall_w: &Vec<Vec<char>>) {
    let dj: Vec<isize> = vec![-1, 0, 1, 0];
    let di: Vec<isize> = vec![0, 1, 0, -1];
    let r#move = vec!['L', 'D', 'R', 'U'];
    visited[i][j] = true;
    for r in 0..4 {
        let ni = i as isize + di[r];
        let nj = j as isize + dj[r];
        if ni < 0 || nj < 0 || ni >= n as isize || nj >= n as isize || visited[ni as usize][nj as usize] {
            continue;
        }
        if (di[r] == 0 && wall_w[i][j.min(nj as usize)] == '0') || (dj[r] == 0 && wall_h[i.min(ni as usize)][j] == '0') {
            print!("{}", r#move[r]);
            dfs(ni as usize, nj as usize, n ,visited, wall_h, wall_w);
            print!("{}", r#move[(r + 2) % 4]);
        }
    }
}

fn solve(){
    input! {
        n: usize,
        wall_h: [Chars; n - 1],
        wall_v: [Chars; n],
        d: [[usize; n]; n],
    }

    dfs(0, 0, n, &mut vec![vec![false; n]; n], &wall_h, &wall_v);
    println!();

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
