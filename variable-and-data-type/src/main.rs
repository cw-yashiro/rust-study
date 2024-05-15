fn main() {
    let message = "message";
    println!("{}", message);

    // mutをつけると再代入が可能
    let mut message = "message";
    println!("{}", message);
    message = "hello World";
    println!("{}", message);

    // 同じ変数名で再度宣言をすることが可能(シャドーイング)
    let message = "hogehoge";
    println!("{}", message);
    let message = "fugafuga";
    println!("{}", message);

    // 定数はconstをつける
    // 定数は型指定が必須
    // 大文字であること推奨されている
    const URL: &str = "http://localhost:5173";
    println!("{}", URL);

    // 文字列型 char
    // 一文字しか表現できない
    let a = 'a';
    println!("{}", a);

    // タプル型
    // .0 .1 のようにアクセスする
    let a = ("hoge", 123);
    println!("{}", a.0);
    println!("{}", a.1);

    // 配列型
    // 固定長しか保持できないところが特徴
    let target = ["hoge", "fuga", "bar"];
    println!("{}", target[0]);
    println!("{}", target[1]);
    println!("{}", target[2]);

    // ベクター型
    // 可変長なコレクションが必要な時はこちらを使う
    let mut v: Vec<i32> = Vec::new();
    v.push(99); // 破壊的なメソッド(データ構造に破壊的な変更をもたらすメソッド)を使うときは mut が必須
    println!("{:?}", v);

    // ベクター型の便利なマクロ
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // 文字列スライス
    // サイズは固定なので変更することはできないが、処理は高速に行うことができる
    let message = "hello world";
    println!("{}", message);

    // String型
    // サイズが可変なので値の更新や結合が可能
    let mut message = String::from("hello world");
    message.push_str(".com");
    println!("{}", message);

    // 文字列スライス → String型
    let message = "hello World".to_string();
    println!("{}", message);

    // 使い分け的には
    // 参照は文字列スライス
    // 文字列のデータ自体が欲しい場合はString型

    // ハッシュマップ
    // キー・バリューストア的な使い方が可能
    // 異なる値は挿入できない
    let mut scores = std::collections::HashMap::new();
    scores.insert("Sato", 100);
    scores.insert("Tanaka", 90);
    println!("{:?}", scores);

    // Tanakaがなかったら100をinsert
    scores.entry("Yamada").or_insert(200);
    println!("{:?}", scores);
}
