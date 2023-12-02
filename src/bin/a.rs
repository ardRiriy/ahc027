/*
このコード、と～おれ!
  ∧＿∧　
（｡･ω･｡)つ━☆・*。
⊂    ノ       ・゜+.
 しーＪ       °。+ *´¨)
                  .· ´¸.·*´¨) ¸.·*¨)
                                      (¸.·´ (¸.·'* ☆
*/


use std::collections::VecDeque;
use std::process::exit;
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

fn back_to_start(i: usize, j: usize, n: usize, walls:&Walls) {
    //{i, j}を始点にBFS
    let inf = 1e18 as usize;
    let mut dist = vec![vec![inf; n]; n];
    let mut before = vec![vec![inf; n]; n];

    let di: Vec<isize> = vec![0, 1, 0, -1];
    let dj: Vec<isize> = vec![-1, 0, 1, 0];

    let r#move = vec!['L', 'D', 'R', 'U'];

    let mut que = VecDeque::new();
    que.push_back((i, j));
    dist[i][j] == 0;
    before[i][j] == 4;
    while let Some((pi, pj)) = que.pop_front() {
        for r in 0..4 {
            let ni = pi as isize + di[r];
            let nj = pj as isize + dj[r];
            if Walls::is_through(walls, pi, pj, ni, nj, n, r) && dist[ni as usize][nj as usize] == inf {
                dist[ni as usize][nj as usize] = dist[pi][pj] + 1;
                before[ni as usize][nj as usize] = r;
                que.push_back((ni as usize, nj as usize));
            }
        }
    }

    let mut stk = Vec::new();
    let mut now_i = 0usize;
    let mut now_j = 0usize;

    while !(now_i == i && now_j == j) {
        stk.push(r#move[before[now_i][now_j]]);
        let ni = (now_i as isize + di[(before[now_i][now_j] + 2) % 4]) as usize;
        let nj = (now_j as isize + dj[(before[now_i][now_j] + 2) % 4]) as usize;

        now_i = ni;
        now_j = nj;
    }

    while let Some(c) = stk.pop() {
        print!("{}", c);
    }
    println!();
    exit(0);
}

fn dfs(i: usize, j: usize, n: usize ,visited: &mut Vec<Vec<bool>>, walls: &Walls) {
    let di: Vec<isize> = vec![0, 1, 0, -1];
    let dj: Vec<isize> = vec![-1, 0, 1, 0];

    let r#move = vec!['L', 'D', 'R', 'U'];
    visited[i][j] = true;

    if visited.iter().all(|v| v.iter().all(|b| *b)) {
        // 即座に0, 0に帰る
        back_to_start(i, j, n, walls);
    }


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


