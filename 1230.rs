fn main() {
    let mut numbers = vec![1, 2, 3];

    // 最初の要素への参照を取得（不変参照）
    let first = &numbers[0];

    println!("追加前の最初の要素: {}", first);

    // 要素を追加しようとする（可変な操作）
    for i in 0..1000 {
        numbers.push(i); // ここでコンパイルエラー！
    }

    // ここで first を使おうとしているため、上記の push は許されない
    println!("追加後の最初の要素: {}", first);
}
