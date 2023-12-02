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

static INF: usize = 1e18 as usize;
static AREAS: usize = 16;
struct Walls {
    wh: Vec<Vec<char>>,
    ww: Vec<Vec<char>>,
}

impl Walls {
    pub fn is_through(&self, i: usize, j: usize, n: usize, r: usize) -> bool {
        let di: Vec<isize> = vec![0, 1, 0, -1];
        let dj: Vec<isize> = vec![-1, 0, 1, 0];

        let ni = i as isize + di[r];
        let nj = j as isize + dj[r];
        if ni < 0 || nj < 0 || ni >= n as isize || nj >= n as isize {
            return false;
        }

        return if (di[r] == 0 && self.ww[i][j.min(nj as usize)] == '0') || (dj[r] == 0 && self.wh[i.min(ni as usize)][j] == '0') {
            true
        } else {
            false
        }
    }
}

/*任意の地点から(0, 0)に戻る*/
fn back_to_start(i: usize, j: usize, n: usize, walls:&Walls) {
    //{i, j}を始点にBFS
    let mut dist = vec![vec![INF; n]; n];
    let mut before = vec![vec![INF; n]; n];

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
            if Walls::is_through(walls, pi, pj, n, r) && dist[ni as usize][nj as usize] == INF {
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

/*
* スタート地点と掃除するエリアを与えて、
* エリア内をDFSする
* 掃除したあとは、必ずはじめの位置に戻る
*/
fn cleanup_area(i: usize, j: usize, n: usize ,visited: &mut Vec<Vec<bool>>, color: &Vec<Vec<usize>>, walls: &Walls) {
    let di: Vec<isize> = vec![0, 1, 0, -1];
    let dj: Vec<isize> = vec![-1, 0, 1, 0];

    let r#move = vec!['L', 'D', 'R', 'U'];
    visited[i][j] = true;

    for r in 0..4 {
        let ni = i as isize + di[r];
        let nj = j as isize + dj[r];

        if Walls::is_through(&walls, i, j, n, r)
                && !visited[ni as usize][nj as usize]
                && color[i][j] == color[ni as usize][nj as usize] {
            print!("{}", r#move[r]);
            cleanup_area(ni as usize, nj as usize, n ,visited, color ,walls);
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

    let di: Vec<isize> = vec![0, 1, 0, -1];
    let dj: Vec<isize> = vec![-1, 0, 1, 0];
    let r#move = vec!['L', 'D', 'R', 'U'];

    // エリア分け
    let mut color = vec![vec![INF; n]; n];
    let mut que = VecDeque::new();
    // TODO: AREASを使って書き換えたい
    for i in 0..4 {
        for j in 0..4 {
            color[(i * n / 4) + 2][(j * n / 4) + 2] = i * 4 + j;
            que.push_back(((i * n / 4) + 2, (j * n / 4) + 2));
        }
    }

    while let Some((p_i, p_j)) = que.pop_front() {
        for r in 0..4 {
            let ni = p_i as isize + di[r];
            let nj = p_j as isize + dj[r];
            if Walls::is_through(&walls, p_i, p_j, n, r) && color[ni as usize][nj as usize] == INF {
                color[ni as usize][nj as usize] = color[p_i][p_j];
                que.push_back((ni as usize, nj as usize));
            }
        }
    }

    /*
    各エリアからの距離を調べる。移動用
    */
    let mut dist_from_area = vec![vec![vec![INF; n]; n];AREAS];
    for clr in 0..AREAS {
        let mut que = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if color[i][j] == clr {
                    dist_from_area[clr][i][j] = 0;
                    que.push_back((i, j));
                }
            }
        }
        while let Some((p_i, p_j)) = que.pop_front() {
            for r in 0..4 {
                let ni = p_i as isize + di[r];
                let nj = p_j as isize + dj[r];
                if Walls::is_through(&walls, p_i, p_j, n, r) && dist_from_area[clr][ni as usize][nj as usize] == INF {
                    dist_from_area[clr][ni as usize][nj as usize] = dist_from_area[clr][p_i][p_j] + 1;
                    que.push_back((ni as usize, nj as usize));
                }
            }
        }
    }

    /*
    エリアごとの汚れやすさを求める(sum_d / cnt)
    平均でやる or 総和でやる ?
       => 平均で
    */
    let mut sum_d = vec![0usize; AREAS];
    let mut cnt = vec![0usize; AREAS];

    for i in 0..n {
        for j in 0..n {
            sum_d[color[i][j]] += d[i][j];
            cnt[color[i][j]] += 1;
        }
    }

    /*
      順番を決める
      はじめ、色0が全部0で、汚れやすさが一番でかいところを掃除しに行く
    */
    let mut dirt = vec![0usize; AREAS];
    let mut permutation = vec![0usize];
    let mut cleaned = vec![false; AREAS]; // 一度でも掃除済みか否かを判定
    cleaned[0] = true;

    for i in 1..16 {
        dirt[i] = sum_d[i];
    }

    while !cleaned.iter().all(|b| *b) {
        // 一番汚れているエリアを探す
        let mut max_idx = 0usize;
        let mut max_dirt = 0usize;
        for i in 0..AREAS {
            if max_dirt < dirt[i] {
                max_dirt = dirt[i];
                max_idx = i;
            }
        }

        cleaned[max_idx] = true;
        permutation.push(max_idx);

        // 汚れを更新
        for i in 0..AREAS {
            if i == max_idx {
                dirt[i] = 0;
            }else{
                dirt[i] += sum_d[i] / cnt[i];
            }
        }
    }
    let mut now_pos = (0usize, 0usize);
    for (idx, &next) in permutation.iter().enumerate() {
        let (mut p_i, mut p_j) = now_pos;
        // まずは掃除をしてもらう
        cleanup_area(p_i, p_j, n, &mut vec![vec![false; n]; n], &color, &walls);

        if idx != permutation.len() - 1 {
            // 最後でなければ次のエリアに移動
            let next_area = permutation[idx + 1];
            while dist_from_area[next_area][p_i][p_j] != 0 {
                // 距離が-1になる場所に移動
                for r in 0..4 {
                    let ni = p_i as isize + di[r];
                    let nj = p_j as isize + dj[r];
                    if Walls::is_through(&walls, p_i, p_j, n, r)
                        && dist_from_area[next_area][ni as usize][nj as usize] + 1 == dist_from_area[next_area][p_i][p_j] {
                        p_i = ni as usize;
                        p_j = nj as usize;
                        print!("{}", r#move[r]);
                        break;
                    }
                }
            }
        }
        now_pos = (p_i, p_j);
    }

    back_to_start(now_pos.0, now_pos.1, n, &walls);
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


