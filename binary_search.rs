// めぐる式二分探索
// main関数の中で使う

let is_ok = |mid: i32| -> bool {
    todo!();
};

let mut ng; // '取り得る最小の値' - 1
let mut ok; // '取り得る最大の値' + 1
while (ok - ng).abs() > 1 {
    let mid = (ok + ng) / 2;
    if is_ok(mid) {
        ok = mid;
    } else {
        ng = mid;
    }
}