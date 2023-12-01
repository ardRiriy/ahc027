/*
このコード、と～おれ!
  ∧＿∧　
（｡･ω･｡)つ━☆・*。
⊂    ノ       ・゜+.
 しーＪ       °。+ *´¨)
                  .· ´¸.·*´¨) ¸.·*¨)
                                      (¸.·´ (¸.·'* ☆
*/


use std::sync::Mutex;
use lazy_static::lazy::Lazy;
// -*- coding:utf-8-unix -*-
use proconio::input;
use proconio::marker::Chars;

struct Walls {
    wh: Vec<Vec<char>>,
    ww: Vec<Vec<char>>,
}

impl Walls {
    pub fn is_through(&self, i: usize, j: usize, ni: isize, nj: isize, n: usize, r: usize) -> bool {
        if ni < 0 || nj < 0 || ni >= n as isize || nj >= n as isize {
            return false;
        }
        let dj: Vec<isize> = vec![-1, 0, 1, 0];
        let di: Vec<isize> = vec![0, 1, 0, -1];
        return if (di[r] == 0 && self.ww[i][j.min(nj as usize)] == '0') || (dj[r] == 0 && self.wh[i.min(ni as usize)][j] == '0') {
            true
        } else {
            false
        }
    }
}

fn dfs(i: usize, j: usize, n: usize ,visited: &mut Vec<Vec<bool>>, walls: &Walls) {
    let dj: Vec<isize> = vec![-1, 0, 1, 0];
    let di: Vec<isize> = vec![0, 1, 0, -1];
    let r#move = vec!['L', 'D', 'R', 'U'];
    visited[i][j] = true;
    for r in 0..4 {
        let ni = i as isize + di[r];
        let nj = j as isize + dj[r];

        if Walls::is_through(&walls, i, j, ni, nj, n, r) && !visited[ni as usize][nj as usize]{
            print!("{}", r#move[r]);
            dfs(ni as usize, nj as usize, n ,visited, walls);
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

    // init
    let walls = Walls{wh: wall_h, ww: wall_v};

    dfs(0, 0, n, &mut vec![vec![false; n]; n], &walls);
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


