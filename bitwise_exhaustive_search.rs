input!{
    n: i32,
}

'outer: for bit in 0..(1 << n) {
    'inner: for i in 0..n {
        if bit & (1 << i) == 0 {
            // i番目を選ばない処理
        }
        else {
            // i番目を選ぶ処理
        }
    }
}