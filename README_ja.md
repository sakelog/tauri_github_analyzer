# tauri_github_analyzer

Tauri で構築した GitHub トラフィックビューア

[README in English](/README.md)

<div align="center">
<img width="640" alt="screenshot_tauri_github_analyzer" src="https://user-images.githubusercontent.com/60056078/165714721-6f1b1003-8d88-4979-87f0-075260459f98.png">
</div>
  
<div align="center">
<img src="https://img.shields.io/github/languages/code-size/sakelog/tauri_github_analyzer" alt="code_size" >
<img src="https://img.shields.io/github/v/release/sakelog/tauri_github_analyzer" alt="release">
<img src="https://img.shields.io/github/issues/sakelog/tauri_github_analyzer" alt="issue">
<img src="https://img.shields.io/github/license/sakelog/tauri_github_analyzer" alt="license">
</div>

<div align="center">
  
  [特徴](#特徴)
  ・
  [必要なもの](#必要なもの)
  ・
  [利用方法](#利用方法)
  ・
  [参考サイト](#参考サイト)
  ・
  [作成者](#作成者)
  ・
  [License](#license)
  
</div>

## 特徴

GitHub の personal access token をアプリのウィンドウに入力するだけで、公開リポジトリの Views と Clones を視覚的に一覧化します。

## 必要なもの

- アプリ利用者
  - Windows 11 (or Windows that installed WebView2)
  - GitHub personal access token
    - public_repo の権限が必要
- 開発者
  - Windows 11 (or Windows that installed WebView2)
  - Microsoft Visual Studio C++ build tools
  - Rustc and Cargo Package Manager
  - Node.js
  - Node.js Package Manager (例： Yarn)

こちらのページも合わせて参照 : https://tauri.studio/docs/getting-started/setting-up-windows

## 利用方法

アプリの最新バージョンはここからダウンロードできます : [Releases](https://github.com/sakelog/tauri_github_analyzer/releases)

初めてアプリを起動すると、token 入力用のモーダルウィンドウが表示されます。

GitHub の personal access token を入力して、Submit ボタンを押してください。

このアプリを動かすには、personal access token の次の権限が必要です。

- public_repo

次回の起動以降は、保存された personal access token を読み込みます。

Windows では、以下のファイルが次のパスに作成されます： C:\Users\{username}\AppData\Roaming\Tauri GitHub Analyzer.

- token.json

token が有効期限切れの場合、モーダルウィンドウが再表示されるので、再度 token を入力してください。

token は最大 5 回まで入力できます。最大入力回数を超えると、アプリは強制終了します。

### 開発者向けの情報

ドキュメントはこちら : https://github.com/sakelog/tauri_github_analyzer/tree/main/docs

下記手順でリポジトリのクローンが可能です。

```shell
git clone ...
cd tauri_github_analyzer
yarn
```

#### デバッグについて

下記コマンドを実行するとデバッグを行うことができます。

```shell
yarn dev
```

#### production build を作成したいとき

production build をするには、下記コマンドを実行してください.

```shell
yarn release
```

## 参考サイト

- [Build smaller, faster, and more secure desktop applications with a web frontend | Tauri Studio](https://tauri.studio/)
- [Introduction - Rust By Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/)
- [reqwest - Rust](https://docs.rs/reqwest/latest/reqwest/)
- [Convert curl commands to code](https://curlconverter.com/#rust)
- [Chakra UI - A simple, modular and accessible component library that gives you the building blocks you need to build your React applications. - Chakra UI](https://chakra-ui.com/)
- [ビルドシステムをcreate-react-appからViteに移行した話 - Sansan Builders Blog](https://buildersbox.corp-sansan.com/entry/2022/03/24/110000)

## 作成者

- sake
  - [Twitter](https://twitter.com/sake_engineer)
  - [GitHub](https://github.com/sakelog)
  - [WebSite](https://sakeengineer.com/)

## License

このプロジェクトは、MIT ライセンスのもとで実施されます。

詳細は [LICENSE](/LICENSE) ファイルを参照。
