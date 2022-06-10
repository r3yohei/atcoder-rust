// argsortする関数
// let sorted_idx = argsort(&vec);
// のように使う
fn argsort<T: Ord>(v: &[T]) -> Vec<usize> {
    let mut idx = (0..v.len()).collect::<Vec<_>>();
    idx.sort_by(|&i, &j| v[i].cmp(&v[j])); // 昇順
    // idx.sort_by(|&i, &j| v[j].cmp(&v[i])); // 降順
    idx
}