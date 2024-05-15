fn main() {
    let domain = "yahoo.co.jp";
    // if式
    let result = if domain == "yahoo.co.jp" {
        "Yahoo だよ"
    } else if domain == "google.com" {
        "Google だよ"
    } else {
        "知らないドメインだよ"
    };
    println!("{}", result);

    // match式
    let result = match domain {
        "yahoo.co.jp" => "Yahoo だよ!",
        "google.com" => "google だよ!",
        _ => "知らないドメインだよ!",
    };
    println!("{}", result);

    // enumのマッチにも使われる
    let data = Some("some text");
    let print_data = match data {
        Some(text) => text,
        None => "None text",
    };
    println!("{}", print_data);

    // v if と続けることでcaseの後にもifを設けることができる

    // loop式
    // breakで抜けることができる
    let mut sum = 0;
    loop {
        sum += 1;
        if sum == 10 {
            break;
        }
        println!("loop@{}", sum);
    }

    // while式
    // ある値を評価してtrueならばループする
    // ループを続けるか判断する場合に利用する
    let mut sum = 0;
    while sum < 10 {
        println!("while loop@{}", sum);
        sum += 1;
    }

    // for式
    let scores = vec![100, 87, 45, 67];
    for score in scores.iter() {
        println!("score is {}", score);
    }
}
