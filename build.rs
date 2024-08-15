fn main() {
    // プロジェクトのルートディレクトリを環境変数として設定
    println!("cargo:rustc-env=RUNNING_WITH_CARGO=1",);
}
