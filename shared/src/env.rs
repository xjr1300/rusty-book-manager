use strum::EnumString;

#[derive(Default, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    /// 開発環境
    #[default]
    Development,
    /// プロダクション環境
    Production,
}

/// アプリケーションを`開発環境かプロダクション環境で動作させるかを判定する。
///
/// 環境のデフォルト値を、debug_assertionsがonの場合は開発環境、それ以外はプロダクション環境と設定する。
/// そして、環境変数ENVによって、動作環境を次の通り判定する。
///
/// * 環境変数ENVが指定されていない場合は、debug_assertionの値で環境を判定
/// * 環境変数EVNが指定されていて、ENVの値が"development"の場合は開発環境と判定
/// * 環境変数ENVが指定されていて、ENVの値が"production"の場合はプロダクション環境と判定
/// * 環境変数ENVが指定されていて、ENVの値が有効でない場合は、環境のデフォルト値で判定
pub fn which() -> Environment {
    #[cfg(debug_assertions)]
    let default_env = Environment::Development;
    #[cfg(not(debug_assertions))]
    let default_env = Environment::Production;

    match std::env::var("ENV") {
        Err(_) => default_env,
        Ok(v) => v.parse().unwrap_or(default_env),
    }
}
