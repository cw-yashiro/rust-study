fn main() {
    let a = String::from("hoge");
    let b = a; // この段階でbに所有権が移っているのでaにはアクセスできない
    println!("{}", b);

    // move と copy
    // moveはデータ自身のコピーはせずに、ポインター(メモリのアドレス)と長さがコピーされる
    // copyはメモリ上で値をコピーする
    // copyトレイトが実装されているものはあまり容量が大きくないため
    // copy → i32型など
    // move → String型など

    // 借用
    // 値の参照を関数などに渡すこと
    fn print_hello(message: &str) {
        println!("{}", message);
    }

    let hello_ja = "こんにちは";
    print_hello(&hello_ja);
    println!("{}", hello_ja);

    // 値のclone
    // 値の所有権が複数の箇所で必要になる場合にcloneを使うことで明示的にコピーできる

    // take(所有権を奪う関数)
    fn take<T>(_value: T) {
        // 所有権を奪うだけ
    }

    let print_data = {
        let result = String::from("hogehoge");
        take(result.clone()); // データのコピーを渡している
        result // 値を返せる
    };
    println!("{}", print_data);
}
