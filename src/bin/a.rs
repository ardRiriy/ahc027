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
use std::mem::swap;
use std::process::exit;
use num_integer::Roots;
// -*- coding:utf-8-unix -*-
use proconio::input;
use proconio::marker::Chars;
use rand::Rng;

static INF: usize = 1e18 as usize;
static AREAS: usize = 16;

static TL: f64 = 1.95;
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


pub fn evaluate(N: usize, d :&Vec<Vec<usize>>, route: &Vec<(usize, usize)>) -> i64 {
    // route => 持ってる
    let mut last_visited = vec![vec![0usize; N]; N];
    let L = route.len();
    let mut S = vec![];

    for t in 0..L {
        last_visited[route[t].0][route[t].1] = t;
    }

    let mut s = 0;
    let mut sum_d = 0;
    for i in 0..N {
        for j in 0..N {
            s += (L - last_visited[i][j]) as i64 * d[i][j] as i64;
            sum_d += d[i][j];
        }
    }
    let mut last_visited2 = last_visited.clone();
    let mut sum = vec![vec![0i64; N]; N];
    for t in L..2 * L {
        let (i, j) = route[t - L];
        let dt = (t - last_visited2[i][j]) as i64;
        let a = dt * d[i][j] as i64;
        sum[i][j] += dt * (dt - 1) / 2 * d[i][j] as i64;
        s -= a;
        last_visited2[i][j] = t;
        S.push(s);
        s += sum_d as i64;
    }

    let score = (2 * S.iter().sum::<i64>() + L as i64) / (2 * L) as i64;
    score

}

#[inline]
fn get_time() -> f64 {  // sec
static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        #[cfg(feature = "local")]
        {
            (ms - STIME) * 0.85
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}

fn debug_grid(v: &Vec<Vec<usize>>) {
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            print!("{:2} ", v[i][j]);
        }
        println!();
    }
}

/*任意の地点から(0, 0)に戻る*/
fn back_to_start(i: usize, j: usize, n: usize, routes: &mut Vec<(usize, usize)> ,walls:&Walls) {
    //{i, j}を始点にBFS
    let mut dist = vec![vec![INF; n]; n];
    let mut before = vec![vec![INF; n]; n];

    let di: Vec<isize> = vec![0, 1, 0, -1];
    let dj: Vec<isize> = vec![-1, 0, 1, 0];

    let mut que = VecDeque::new();
    que.push_back((i, j));
    dist[i][j] = 0;
    before[i][j] = 4;
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
        stk.push((now_i, now_j));
        let ni = (now_i as isize + di[(before[now_i][now_j] + 2) % 4]) as usize;
        let nj = (now_j as isize + dj[(before[now_i][now_j] + 2) % 4]) as usize;

        now_i = ni;
        now_j = nj;
    }

    while let Some(c) = stk.pop() {
        routes.push(c);
    }
}

/*
* スタート地点と掃除するエリアを与えて、
* エリア内をDFSする
* 非再起で書いて、掃除終了時点の座標を返却
*/
fn cleanup_area(i: usize, j: usize, n: usize, color: &Vec<Vec<usize>>, routes: &mut Vec<(usize, usize)>, dirts: &mut Vec<Vec<usize>>, d: &Vec<Vec<usize>> ,walls: &Walls) -> (usize, usize) {
    let di: Vec<isize> = vec![0, 1, 0, -1];
    let dj: Vec<isize> = vec![-1, 0, 1, 0];

    let mut pos = (i, j); // 今の位置

    /* DFS */
    let mut visited = vec![vec![false; n]; n];
    visited[i][j] = true;

    let mut stk = Vec::new();
    for r in 0..4usize {
        let ni = i as isize+ di[r];
        let nj = j as isize + dj[r];
        if Walls::is_through(walls, i, j, n, r) && color[ni as usize][nj as usize] == color[i][j] {
            stk.push((ni, nj, r));
        }
    }

    // dirの向きに移動した結果{p_i, p_j}に到達した
    'dfs : while let Some((p_i, p_j, dir)) = stk.pop() {
        if p_i >= 0 {
            if !visited[p_i as usize][p_j as usize] { // 未訪問なら訪れる
                //println!("#{} : {} {} {}", color[i][j], p_i, p_j, dir);

                visited[p_i as usize][p_j as usize] = true;
                pos = (p_i as usize, p_j as usize);
                routes.push((p_i as usize, p_j as usize));
                update_dirt(dirts, d, &mut vec![0usize; color.len()], color, (p_i as usize, p_j as usize));


                // 帰りがけの処理を追加する
                // 帰りがけはbit反転
                stk.push((!(p_i + di[(dir + 2) % 4]), p_j + dj[(dir + 2) % 4], (dir + 2) % 4));

                // 行きがけの処理を追加する
                for r in 0..4 {
                    let ni = p_i + di[r];
                    let nj = p_j + dj[r];

                    if Walls::is_through(walls, p_i as usize, p_j as usize, n, r) && color[ni as usize][nj as usize] == color[i][j] {
                        stk.push((ni, nj, r));
                    }
                }

                // もしエリアをすべて探索しきっていた場合は打ち切って今の座標を帰す
                for idx in 0..n {
                    for jdx in 0..n {
                        if color[i][j] == color[idx][jdx] && !visited[idx][jdx] {
                            continue 'dfs;
                        }
                    }
                }
                return pos;
            }
        }else{
            // 帰りがけは絶対出力
            pos = (!p_i as usize, p_j as usize);
            routes.push((!p_i as usize, p_j as usize));
            update_dirt(dirts, d, &mut vec![0usize; color.len()], color, (!p_i as usize, p_j as usize));
        }
    }
    pos
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
    let mut rng = rand::thread_rng();

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
    各頂点からの距離をそれぞれ調べる
    */
    let mut dist_from_point = vec![vec![vec![INF; n]; n]; n * n];
    for i in 0..n {
        for j in 0..n {
            let idx = i * n + j;

            dist_from_point[idx][i][j] = 0;
            let mut que = VecDeque::new();
            que.push_back((i, j));
            while let Some((p_i, p_j)) = que.pop_front() {
                for r in 0..4 {
                    let ni = p_i as isize + di[r];
                    let nj = p_j as isize + dj[r];
                    if Walls::is_through(&walls, p_i, p_j, n, r) && dist_from_point[idx][ni as usize][nj as usize] == INF {
                        dist_from_point[idx][ni as usize][nj as usize] = dist_from_point[idx][p_i][p_j] + 1;
                        que.push_back((ni as usize, nj as usize));
                    }
                }
            }
        }
    }

    /*
    エリアごとの汚れやすさを求める(sum_d / cnt)
    平均でやる or 総和でやる ?
       => 平均で
    */
    let mut cnt = vec![0usize; AREAS];

    for i in 0..n {
        for j in 0..n {
            cnt[color[i][j]] += 1;
        }
    }

    /*
      順番を決める
      はじめ、色0が全部0で、汚れやすさが一番でかいところを掃除しに行く
    */
    let mut dirt = vec![0usize; AREAS];
    let mut cleaned = vec![false; AREAS]; // 一度でも掃除済みか否かを判定
    let mut room_dirt = vec![vec![0usize; n]; n];
    let mut tracking_route = vec![(0usize, 0usize)]; // 到達順番を追跡

    let mut now_pos = (0usize, 0usize);
    while !cleaned.iter().all(|b| *b) {
        update_dirt(&mut room_dirt, &d, &mut dirt, &color, now_pos);

        // 一番汚れているエリアを探す
        let mut max_idx = 0usize;
        let mut max_dirt = 0usize;
        for i in 0..AREAS {
            if max_dirt < dirt[i] / cnt[i] {
                max_dirt = dirt[i] / cnt[i];
                max_idx = i;
            }
        }

        // 次のエリアに移動
        while dist_from_area[max_idx][now_pos.0][now_pos.1] != 0 {
            // 距離が-1になる場所に移動
            for r in 0..4 {
                let ni = now_pos.0 as isize + di[r];
                let nj = now_pos.1 as isize + dj[r];
                if Walls::is_through(&walls, now_pos.0, now_pos.1, n, r)
                    && dist_from_area[max_idx][ni as usize][nj as usize] + 1 == dist_from_area[max_idx][now_pos.0][now_pos.1] {
                    now_pos.0 = ni as usize;
                    now_pos.1 = nj as usize;
                    tracking_route.push((now_pos.0, now_pos.1));
                    update_dirt(&mut room_dirt, &d, &mut dirt, &color, now_pos);
                    break;
                }
            }
        }
        // max_idxを掃除
        now_pos = cleanup_area(now_pos.0, now_pos.1, n, &color, &mut tracking_route, &mut room_dirt, &d ,&walls);
        cleaned[max_idx] = true;
    }

    back_to_start(now_pos.0, now_pos.1, n, &mut tracking_route ,&walls);


    let mut prev_score = evaluate(n, &d, &tracking_route);

    // 現時点でtracking_routeが初期状態
    // 終了時点で全箇所が通ってないと困るので、通った回数を記録しておく
    let mut passing_times = vec![vec![0usize; n]; n];
    for i in 0..tracking_route.len() {
        passing_times[tracking_route[i].0][tracking_route[i].1] += 1;
    }

    let mut cnt = 0usize;

    let start_temp = 0isize;
    let end_temp = 0isize;
    'annealing: while get_time() < TL {
        // memo: idxとidx + rangeは、"接続先"であって、ここは変えない
        let idx = rng.gen_range(0..tracking_route.len());
        let range = rng.gen_range(5..100); // 現状の何手先まで変えるか 値は適当. 最後にidx+rangeに接続できないと行けない

        /* その区間を変更していいか判定 */
        let mut passed_in_range = vec![vec![0usize; n]; n];
        if idx + range >= tracking_route.len()-1 { // 最後が変わると困るので、-1してる
            continue 'annealing;
        }
        for i in idx..idx+range {
            passed_in_range[tracking_route[i].0][tracking_route[i].1] += 1;
            if passing_times[tracking_route[i].0][tracking_route[i].1] == passed_in_range[tracking_route[i].0][tracking_route[i].1] {
                // ここが変わるとinvalidな解になるので、やらない
                continue 'annealing;
            }
        }

        cnt += 1;


        // 新しいルートは、とりあえずBFS。基本的には短いほうがいいので。
        // 実際に改善するかどうかは、あとで分かるのでなんでもいいのです。
        // ↑本当はどうでも良くはないんだけど、まぁまだあと5日あるので多少は...
        
        let mut pos = tracking_route[idx + range];
        let mut new_route = tracking_route.clone();

        let mut update = vec![];
        while dist_from_point[tracking_route[idx].0 * n + tracking_route[idx].1][pos.0][pos.1] != 0 {
            // 距離が-1になる場所に移動
            for r in 0..4 {
                let ni = pos.0 as isize + di[r];
                let nj =pos.1 as isize + dj[r];
                if walls.is_through(pos.0, pos.1, n, r)
                    && dist_from_point[tracking_route[idx].0 * n + tracking_route[idx].1][ni as usize][nj as usize] + 1 == dist_from_point[tracking_route[idx].0 * n + tracking_route[idx].1][pos.0][pos.1] {
                    pos.0 = ni as usize;
                    pos.1 = nj as usize;
                    update.push(pos);
                    break;
                }
            }
        }

        update.reverse();
        new_route.splice(idx+1..idx+range, update.clone());
        let new_score = evaluate(n, &d, &new_route);

        let temp = start_temp as f64 + (end_temp - start_temp) as f64 * get_time() / TL;
        let prob = ((prev_score - new_score) as f64 / temp).exp();

        if prob > (rng.gen_range(0..INF) % INF) as f64 / INF as f64 {
            // 改善しているのなら採用
            prev_score = new_score;

            for p in update {
                passing_times[p.0][p.1] += 1;
            }
            for i in idx+1..idx+range{
                passing_times[tracking_route[i].0][tracking_route[i].1] -= 1;
            }

            swap(&mut tracking_route, &mut new_route);
        }

    }

    // tracking_routeを元に答えを出力
    for i in 0..tracking_route.len()-1 {
        let dy = tracking_route[i+1].0 as isize - tracking_route[i].0 as isize;
        let dx = tracking_route[i+1].1 as isize - tracking_route[i].1 as isize;
        for r in 0..4 {
            if di[r] == dy && dj[r] == dx {
                print!("{}", r#move[r]);
                break;
            }
        }
    }
    println!();
    println!("{}", cnt);
}

fn update_dirt(dirts: &mut Vec<Vec<usize>>, d: &Vec<Vec<usize>>, area_dirt: &mut Vec<usize>, color: &Vec<Vec<usize>>, p: (usize, usize)) {
    area_dirt.fill(0);

    // 汚れの更新
    for i in 0..dirts.len() {
        for j in 0..dirts[i].len() {
            dirts[i][j] += d[i][j].sqrt();
            area_dirt[color[i][j]] += dirts[i][j];
        }
    }

    // 今いる地点は掃除されるので値は0に
    area_dirt[color[p.0][p.1]] -= dirts[p.0][p.1];
    dirts[p.0][p.1] = 0;
}


fn main() {
    let mut i: usize = 1;
    get_time();
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


