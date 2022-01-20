# コンパイル時レイトレーシング in Rust  
Rustでコンパイル時レイトレーシングを行うリポジトリです。  
コンパイル時にすべての計算を行い実行時には画像出力を行うだけです。  

`const fn`の中で浮動小数点演算を行うためにnightlyビルドで`#![feature(const_fn_floating_point_arithmetic)]`を有効にしています。

## レンダリング結果
![Listing1](./output/listing1.png)