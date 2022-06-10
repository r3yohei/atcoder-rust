// DFS
// main関数内で使用するときはよく，dfs(start, -1, グラフの隣接リスト表現, 訪問フラグ)
// のようにする
fn dfs(current: usize, prev: usize, edges: &Vec<Vec<usize>>, mut visited: &mut Vec<bool>) {
    visited[current] = true;
    // 行きがけにやりたいことをここに書く
    for &to in &edges[current] {
        // すでに見た頂点を行き先に設定しない
        if visited[to] {
            continue;
        }
        dfs(to, current, edges, visited);
    }
    // 帰りがけにやりたいことをここに書く
}