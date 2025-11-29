use std::sync::{Arc, Mutex};

/// ポートフォリオデータを取得するURL
pub(crate) const PORTFOLIO_URL: &str =
    "https://raw.githubusercontent.com/pirakansa/Gridelle_example/refs/heads/main/portfolio.yaml";

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub(crate) struct RepoSummary {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) image_url: Option<String>,
    #[serde(default)]
    pub(crate) badges: Option<String>,
    pub(crate) updated_at: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq, Default)]
pub(crate) struct SectionMeta {
    #[serde(rename = "rowKey", default)]
    pub(crate) row_key: Option<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub(crate) struct RepoSection {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) meta: SectionMeta,
    pub(crate) items: Vec<RepoSummary>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub(crate) struct FeaturedRepo {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) repository: String,
    pub(crate) tags: Vec<String>,
    pub(crate) updated_at: String,
}

pub(crate) fn load_featured_repo() -> FeaturedRepo {
    const FEATURED_YAML: &str = include_str!("../../assets/featured.yaml");

    serde_yaml::from_str::<FeaturedRepo>(FEATURED_YAML).expect("featured YAML should be valid")
}

/// ポートフォリオデータのロード状態
#[derive(Clone, Default)]
pub(crate) enum PortfolioLoadState {
    #[default]
    NotStarted,
    Loading,
    Loaded(Vec<RepoSection>),
    Error(String),
}

/// 非同期でポートフォリオデータを取得するためのハンドラ
#[derive(Clone, Default)]
pub(crate) struct PortfolioLoader {
    state: Arc<Mutex<PortfolioLoadState>>,
}

impl PortfolioLoader {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PortfolioLoadState::NotStarted)),
        }
    }

    /// 現在の状態を取得します
    pub(crate) fn state(&self) -> PortfolioLoadState {
        self.state.lock().unwrap().clone()
    }

    /// URLからポートフォリオデータの取得を開始します
    pub(crate) fn start_loading(&self, ctx: &egui::Context) {
        {
            let mut state = self.state.lock().unwrap();
            if !matches!(*state, PortfolioLoadState::NotStarted) {
                return;
            }
            *state = PortfolioLoadState::Loading;
        }

        let state = self.state.clone();
        let ctx = ctx.clone();

        ehttp::fetch(
            ehttp::Request::get(PORTFOLIO_URL),
            move |result: ehttp::Result<ehttp::Response>| {
                let new_state = match result {
                    Ok(response) => {
                        if response.ok {
                            match response.text() {
                                Some(text) => {
                                    match serde_yaml::from_str::<Vec<RepoSection>>(text) {
                                        Ok(sections) => PortfolioLoadState::Loaded(sections),
                                        Err(e) => PortfolioLoadState::Error(format!(
                                            "YAML parse error: {e}"
                                        )),
                                    }
                                }
                                None => PortfolioLoadState::Error(
                                    "Response body is not valid UTF-8".to_string(),
                                ),
                            }
                        } else {
                            PortfolioLoadState::Error(format!(
                                "HTTP error: {} {}",
                                response.status, response.status_text
                            ))
                        }
                    }
                    Err(e) => PortfolioLoadState::Error(format!("Network error: {e}")),
                };

                *state.lock().unwrap() = new_state;
                ctx.request_repaint();
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_featured_repo_reads_dedicated_yaml() {
        let data = load_featured_repo();
        assert!(
            !data.name.is_empty(),
            "featured data should load from its dedicated YAML"
        );
    }
}
