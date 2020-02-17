use std::io::Write;
use std::str::FromStr;

fn main() {
    // C++の std::vector, Pythonのリストに相当し、ミュータブルに使いたいため mut キーワードが必要
    let mut numbers = Vec::new();

    // std::env::args はイテレータを返す（skip(1)は最初を取り除いた新たなイテレータを返す）
    for arg in std::env::args().skip(1) {
        // u64 型は FromStr を実装しており u64::from_str でコマンドライン引数をパースして整数にしている
        // パース結果は Result 値で返るため expect でチェックし Ok だったら値が取り出せる
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        // writeln! マクロは内部で Write トレイトを実装した write_fmt メソッドを持つ std::io::Stderr 型を使っている
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap(); // unwarp は expect の手抜きな書き方
        // 引数パースした結果が0件だった時はプログラムを終了する
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // numbers の2番目以降の要素の所有権を & で参照し借用する
    for m in &numbers[1..] {
        // 借りている m の参照先の値 * を渡す
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}

