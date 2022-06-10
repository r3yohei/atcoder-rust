// BFS
// 距離がすべて同じグラフの単一始点最短経路問題に使用する
use std::collections::VecDeque;

// よくある入力
input!{
    n: usize,
    m: usize
    ab: [(usize, usize); m],
}

// 隣接リスト表現
let mut edges = vec![vec![]; n];
for i in 0..m {
    let (a, b) = ab[i];
    edges[a-1].push(b-1);
    edges[b-1].push(a-1);
}

// 頂点0を視点とするBFS
let mut deque = VecDeque::new();
deque.push_back(0_usize);
let mut dist = vec![-1; n]; // -1は未訪問を示す
// 頂点0から各頂点への最短距離を格納するベクタ
dist[0] = 0; // 始点自身への距離は0
while !deque.is_empty() {
    let frm = deque.pop_front().unwrap();
    for &to in &edges[frm] {
        if dist[to] == -1 {
            dist[to] = dist[frm] + 1;
            deque.push_back(to);
        }
    }
}