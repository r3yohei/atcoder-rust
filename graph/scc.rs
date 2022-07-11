// 強連結成分分解
// 使い方
// 隣接リスト表現のグラフedgesをいつも通り用意する
// let scc = SCC::new(&edges);
// scc.groupsの中に，強連結成分ごとの頂点番号が格納される

struct SCC {
    groups: Vec<HashSet<usize>>, k: usize //k個の互いに行き来できる頂点集合
}
impl SCC {
    pub fn new(g: &Vec<Vec<usize>>) -> Self {
        let n = g.len();
        let mut groups: Vec<HashSet<usize>> = vec![HashSet::new(); 0];
        let mut post_order = Vec::new();
        let mut used1 = vec![false; n];
        for v in 0..n {
            if used1[v] { continue; }
            Self::dfs1(&g, &mut post_order, &mut used1, v);
        }
        post_order.reverse();
        let mut rev_g = vec![Vec::new(); n];
        let mut used2 = vec![false; n];
        for v in 0..n {
            for &nxt in &g[v] {
                rev_g[nxt].push(v);
            }
        }
        for &v in &post_order {
            if used2[v] { continue; }
            let mut res = HashSet::new();
            Self::dfs2(&rev_g, &mut used2, &mut res, v);
            groups.push(res);
        }
        let k = groups.len();
        Self {
            groups, k
        }
    }
    fn dfs1 (g: &Vec<Vec<usize>>, post_order: &mut Vec<usize>, used: &mut Vec<bool>, v: usize) {
        used[v] = true;
        for &nxt in &g[v] {
            if used[nxt] { continue; }
            Self::dfs1(g, post_order, used, nxt);
        }
        post_order.push(v);
    }
    fn dfs2 (rev_g: &Vec<Vec<usize>>, used: &mut Vec<bool>, res: &mut HashSet<usize>, v: usize) {
        res.insert(v);
        used[v] = true;
        for &nxt in &rev_g[v] {
            if used[nxt] { continue; }
            Self::dfs2(rev_g, used, res, nxt);
        }
    }
}