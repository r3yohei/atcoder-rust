// DFS
// dfs(0, std::usize::MAX, &edges, &mut visited);
fn dfs(cur: usize, pre: usize, edges: &Vec<Vec<usize>>, mut visited: &mut Vec<bool>) {
    visited[cur] = true;
    // 行きがけにやりたいことをここに書く
    for &to in &edges[cur] {
        // すでに見た頂点を行き先に設定しない
        if visited[to] {
            continue;
        }
        dfs(to, cur, edges, visited);
    }
    // 帰りがけにやりたいことをここに書く
}