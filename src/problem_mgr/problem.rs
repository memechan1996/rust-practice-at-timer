use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Problem{
    /// IRTモデルのスロープ（傾き）。推定できない問題はnull
    pub slope: Option<f64>,

    /// IRTモデルの切片。推定できない問題はnull
    pub intercept: Option<f64>,

    /// IRTモデルの分散。推定できない問題はnull
    pub variance: Option<f64>,

    /// 難易度（レーティング換算）。推定できない問題はnull
    pub difficulty: Option<f64>,

    /// 識別力
    pub discrimination: Option<f64>,

    /// IRTの対数尤度
    pub irt_loglikelihood: Option<f64>,

    /// IRTに使われたユーザ数
    pub irt_users: Option<u32>,

    /// 実験的推定かどうか
    pub is_experimental: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct ProblemInfo{
    pub id: String,
    pub contest_id: String,
    pub title: String,
}