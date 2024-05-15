fn main() {
    println!("Hello, world!");

    // 戻り値に ; をつけない場合は値が返らない
    let f1 = if true {
        let a = 1;
        a
    }else {
        let b = 2;
        b
    };
    println!("{}", f1);

    // enum
    // いくつかのデータを人まとまりにして1つの型として扱うことができる
    // 取りうる値を全て列挙できる

    // Option
    // 値を取り出す際に値があるかどうかを確認する必要がある(nullチェックもれなどのバグを減らすことができる)
    
    // Result
    // エラーを戻り値として扱うことができる

    // ?演算子
    // エラーが起きた時に即returnしたい場面で使用する
    fn always_error() -> Result<(), String> {
        Err("常にエラー出すよ".to_string())
    }

    fn might_fail() -> Result<(), String> {
        let _result = always_error()?; // ここでErrを返す
        Ok(())
    }

    let message = match might_fail() {
        Ok(_) => "処理成功しやした".to_string(),
        Err(cause_message) => cause_message,
    };

    println!("{}", message);

    // panic!マクロ
    // panic!でプログラムを終了させることができる

    // unwrap/expect
    // ResultやOptionから値を取り出すときに必ず値が入っている
    // もしくは、値がなかったらpanicさせたいケースの時に使う

    // つまりResultやOptionから値を取り出したい時に使う
    // unwrapとexpectの違いは
    // expectはpanicする前にエラーメッセージを吐いて取り出す
    let input: Result<&str, &str> = Err("test");
    let input = input.unwrap();
    println!("{:?}", input);
}
