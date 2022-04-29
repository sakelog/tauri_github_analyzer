# tauri_github_analyzer

Viewer of GitHub traffic built in Tauri.

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
  
  [Feature](#feature)
  ・
  [Requirement](#requirement)
  ・
  [Usage](#usage)
  ・
  [Reference](#reference)
  ・
  [Author](#author)
  ・
  [License](#license)
  
</div>

## Feature

Just enter the personal access token in this app window to graphically list the Views and Clones of your public repositories.

## Requirement

- for Users
  - Windows 11 (or Windows that installed WebView2)
  - GitHub personal access token
    - permission : public_repo
- for Developers
  - Windows 11 (or Windows that installed WebView2)
  - Microsoft Visual Studio C++ build tools
  - Rustc and Cargo Package Manager
  - Node.js
  - Node.js Package Manager (e.g. Yarn)

Check also this page : https://tauri.studio/docs/getting-started/setting-up-windows

## Usage

You can download the latest version here : [Releases](https://github.com/sakelog/tauri_github_analyzer/releases)

When the application is launched for the first time, a modal window appears.

Enter your GitHub personal access token and click the Submit button in the modal window.

This app requires the following permissions of GitHub personal access token.

- public_repo

The next time, this app will load the saved personal access token.

In Windows, the following file will be created in C:\Users\{username}\AppData\Roaming\Tauri Github Analyzer.

- token.json

If the token has expired, a modal window will reappear , please enter a new token.

Token can be re-entered up to 5 times. If the limit is exceeded, the application will be forced to exit.

### For Developpers

Docs is here : https://github.com/sakelog/tauri_github_analyzer/tree/main/docs

You can clone this repository with follow command.

```shell
git clone ...
cd tauri_github_analyzer
yarn
```

#### About debug

You can also debug with the following commands.

```shell
yarn dev
```

### If you make a production build

For production build, execute the following command.

```shell
yarn release
```

## Reference

- https://tauri.studio/
- https://doc.rust-jp.rs/rust-by-example-ja/
- https://docs.rs/reqwest/latest/reqwest/
- https://curlconverter.com/#rust
- https://chakra-ui.com/

## Author

- sake
  - [Twitter](https://twitter.com/sake_engineer)
  - [GitHub](https://github.com/sakelog)
  - [WebSite](https://sakeengineer.com/)

## License

This project is licensed under the MIT License.

See the [LICENSE](/LICENSE) file for details.
