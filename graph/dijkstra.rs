// ダイクストラ法
use std::collections::BinaryHeap;
use std::cmp::Reverse;
const INF: u128 = std::u128::MAX;

// s: 始点(0-index)
// n: 頂点数
// edges: グラフの隣接リスト表現(0-index, 重み付き)
fn dijkstra(s: usize, n: usize, edges: &Vec<Vec<(u128, usize)>>) -> Vec<u128> {
    let mut dist = vec![INF; n];
    dist[s] = 0;
    let mut bh = BinaryHeap::new();
    bh.push((Reverse(0), s));
    while let Some((Reverse(dist_to_frm), frm)) = bh.pop() {
        // 見ようとしているものより既にいい経路が見つかっていれば飛ばす
        if dist[frm] < dist_to_frm {continue;}
        for &(cost, to) in &edges[frm] {
            // 更新したほうがいいなら更新して，優先度付きキューに入れる
            if dist_to_frm + cost < dist[to] {
                dist[to] = dist[frm] + cost;
                // rustの優先度付きキューは最大値を取り出すので，負の値で管理
                bh.push((Reverse(dist[to]), to));
            }
        }
    }
    dist
}

/*
// よくある入力と，0始点のdijkstra
input!{
    n: usize,
    m: usize,
    abc: [(usize, usize, u128); m],
}
// 1からnへのダイクストラをすると，すべての頂点への1を始点とする最短経路が出せる
// また，nから1へのダイクストラをすると，すべての頂点へのnを始点とする最短経路が出せる
// これらはそれぞれO((e+n)logn)で，前計算可能
// ゆえに，任意のkについて，dist_from_1[k] + dist_from_n[k]をO(1)で出せるようになる

// 隣接リスト表現
let mut edges = vec![vec![]; n];
for i in 0..m {
    let (a, b, c) = abc[i];
    edges[a-1].push((c, b-1));
    edges[b-1].push((c, a-1));
}

let dist = dijkstra(0, n, &edges);
*/