use std::env;

fn main() {
    // プロジェクトのルートディレクトリを環境変数として設定
    println!(
        "cargo:rustc-env=PROJECT_ROOT={}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}
