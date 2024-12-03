# rusty-book-manager-template

『Rust による Web アプリケーション開発』上の蔵書管理アプリケーションを実装するために便利なテンプレートリポジトリです。本書の開発を開始するにあたり、一通り必要な設定やファイルを揃えています。

このテンプレートリポジトリをお手元に clone することで、本書での開発をよりスムーズに進めることができます。このリポジトリを利用して開発を進めると、cargo-make に関する設定や Docker に関連する設定をショートカットすることができます。Rust のコードに集中して本書の理解を進めたい方はぜひ、このリポジトリをお手元に clone してください。

## 使い方

このリポジトリのページの右上にある「Use this repository」というボタンを押してください。ボタンを押下後、「Create a new repository」を選択します。

![Use this repository](./doc/images/readme_image_1.png)

続いて、自身のリポジトリ名を入力し、リポジトリを作成します。手元にリポジトリが作成されたら、開発を開始することができます。

![Create a new repository](./doc/images/readme_image_2.png)

## テンプレートの内容

このテンプレートリポジトリには下記が含まれています。

- Dockerfile: Docker 向けの設定が書かれています。
- compose.yaml: docker compose を立ち上げるために必要です。
- Makefile.toml: cargo-make の設定のために必要です。
- rust-toolchain.toml: 書籍と Rust のバージョンをそろえるために必要です。
- .github: GitHub Actions の設定ファイルが主に含まれています。
- infra: AWS 上にリソースを構築し、デプロイやリリースを一通り体験したい場合に必要です。セットアップの方法については、[README](./infra/README.md)に記載しています。
- frontend: rust-book-manager の UI を立ち上げたい場合に必要です。立ち上げる方法は、[README](./frontend/README.md)に記載しています。

## リクエスト例

```sh
# アクセストークンのリクエスト
curl --include -H 'Content-Type: application/json' -d '{"email":"eleazar.fig@example.com","password":"password"}' http://localhost:8080/auth/login

# 蔵書の登録リクエスト
# アクセストークン: 6a2c7e0af361414fb787ee8ac56f40a8
curl --include -X POST -H 'Content-Type: application/json' -H 'Authorization: Bearer 6a2c7e0af361414fb787ee8ac56f40a8' -d '{"title":"RustによるWebアプリケーション開発","author":"豊田優貴／松本健太郎／吉川哲史","isbn":"978-4-06-536957-9","description":"Rustによるアプリケーション開発のベストプラクティス!\\n経験豊富な筆者が貴重な知識とテクニックを伝授。「蔵書管理アプリケーション」の実装を通じて、設計、開発、保守、運用までハンズオンで学ぶ!\\n今こそ現場にRustを!"}' http://localhost:8080/api/v1/books

# 蔵書リストの取得リクエスト
curl --include -H 'Authorization: Bearer 6a2c7e0af361414fb787ee8ac56f40a8' http://localhost:8080/api/v1/books

# 蔵書を借りるリクエスト
# 蔵書ID: 5490901bdab84bdaa96090b2647d5887
curl --include -X POST -H 'Authorization: Bearer 6a2c7e0af361414fb787ee8ac56f40a8' http://localhost:8080/api/v1/books/5490901bdab84bdaa96090b2647d5887/checkouts

# 貸出中蔵書リストの取得リクエスト
curl --include -H 'Authorization: Bearer 6a2c7e0af361414fb787ee8ac56f40a8' http://localhost:8080/api/v1/books/checkouts

# 貸出中の蔵書を返却するリクエスト
# 貸出ID: cb7823cb1eee48a99e34b74392637320
curl --include -X PUT -H 'Authorization: Bearer 6a2c7e0af361414fb787ee8ac56f40a8' http://localhost:8080/api/v1/books/5490901bdab84bdaa96090b2647d5887/checkouts/cb7823cb1eee48a99e34b74392637320/returned
```
