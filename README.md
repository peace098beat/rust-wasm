
# Rust + WebAssembly

## 参考リンク

### Rustupを使った方法

 - [Rustで初めての簡単なWebAssembly入門 - Qiita](https://qiita.com/shikigamix/items/152bbd8ca99172ef5ab7)
   - cargoでbuildする方法
 - [Rust + WebAssemblyで広がるWebの未来 - Speaker Deck](https://speakerdeck.com/likr/rust-plus-webassemblydeguang-garuwebfalsewei-lai?slide=19)
   - rustupだけでbuildする方法

### 公式っぽいリンク

wasm-pack と Node.js を使った参考

 - [Rust から WebAssembly にコンパイルする - WebAssembly | MDN](https://developer.mozilla.org/ja/docs/WebAssembly/Rust_to_wasm)
 - [Introduction - Rust and WebAssembly](https://rustwasm.github.io/docs/book/)

 - [初めてRustでWebAssemblyするときに紹介したいチュートリアル (入門) - Qiita](https://qiita.com/kamykn/items/371cba5487d3c7cea8aa)


## 環境

```bash
$ rustup -V
rustup 1.18.3 (435397f48 2019-05-22)

$ cargo -V
cargo 1.35.0 (6f3e9c367 2019-04-04)
```

## Tips

ビルドターゲットの追加

```bash
$ rustup target add wasm32-unknown-unknown
```

ビルド

```bash
$ cargo build --target wasm32-unknown-unknown
```


webサーバー

WEBサーバーには ** Web Server for Chrome ** を利用. 

![](https://lh3.googleusercontent.com/j5Qh64sO4UGPG3yaNELSwCbk1ZraNxFyVly2W5Qz9IpZUZ5Xvo6_jpF-E6PLzdj_u4RRre90pw=w128-h128-e365)

[Chrome ウェブストア - 拡張機能](https://chrome.google.com/webstore/detail/web-server-for-chrome/ofhbbkphhbklhfoeikjpcbhemlocgigb)



@tomoyuki_nohara