
fn main() {
    let a = 123;
    let p1 = &a;         // 123という値が格納された領域への参照をpに代入する
    println!("{}", *p1); // pが参照する領域の値(123)を出力する


    let b = 123;
    let ref p2 = b;
    println!("{}", *p2); // => 123


    let mut c = 123;    // ミュータブルな変数aを定義
    let p3 = &mut c;     // ミュータブルな参照pを定義
    *p3 = 456;           // 参照先の値を456に書き換える
    println!("{}", c);	// => 456
}