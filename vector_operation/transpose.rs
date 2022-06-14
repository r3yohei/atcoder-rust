// 二次元配列の転置を求める
fn transpose(a: &Vec<Vec<i128>>) -> Vec<Vec<i128>>{
    let h = a.len();
    let w = a[0].len();
    let mut a_t = vec![vec![0; h]; w];
    for hi in 0..h {
        for wi in 0..w {
            a_t[wi][hi] = a[hi][wi];
        }
    }
    a_t
}