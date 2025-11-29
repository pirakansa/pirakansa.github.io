use std::sync::{Arc, Mutex};

/// ポートフォリオデータを取得するURL
pub(crate) const PORTFOLIO_URL: &str = "https://raw.githubusercontent.com/pirakansa/Gridelle_example/refs/heads/main/portfolio.yaml";

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub(crate) struct RepoSummary {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) image_url: Option<String>,
    pub(crate) repo_url: String,
    #[serde(default)]
    pub(crate) badges: Option<String>,
    #[serde(default)]
    pub(crate) updated_at: Option<String>,
}

impl RepoSummary {
    /// Returns true if this repository matches the given search query.
    /// Matches against name, description, and badges (case-insensitive).
    pub(crate) fn matches_query(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let query_lower = query.to_lowercase();
        if self.name.to_lowercase().contains(&query_lower) {
            return true;
        }
        if self.description.to_lowercase().contains(&query_lower) {
            return true;
        }
        if let Some(badges) = &self.badges {
            if badges.to_lowercase().contains(&query_lower) {
                return true;
            }
        }
        false
    }
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

impl RepoSection {
    /// Returns a new section containing only items that match the query.
    /// If query is empty, returns all items.
    pub(crate) fn filter_by_query(&self, query: &str) -> RepoSection {
        RepoSection {
            name: self.name.clone(),
            meta: self.meta.clone(),
            items: self
                .items
                .iter()
                .filter(|item| item.matches_query(query))
                .cloned()
                .collect(),
        }
    }
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

    #[test]
    fn matches_query_by_name() {
        let repo = RepoSummary {
            name: "Test Repo".to_string(),
            description: "A sample description".to_string(),
            image_url: None,
            repo_url: "https://example.com".to_string(),
            badges: None,
            updated_at: None,
        };
        assert!(repo.matches_query("test"));
        assert!(repo.matches_query("Repo"));
        assert!(!repo.matches_query("missing"));
    }

    #[test]
    fn matches_query_by_description() {
        let repo = RepoSummary {
            name: "Example".to_string(),
            description: "A Rust project".to_string(),
            image_url: None,
            repo_url: "https://example.com".to_string(),
            badges: None,
            updated_at: None,
        };
        assert!(repo.matches_query("rust"));
        assert!(repo.matches_query("project"));
    }

    #[test]
    fn matches_query_by_badges() {
        let repo = RepoSummary {
            name: "Example".to_string(),
            description: "Description".to_string(),
            image_url: None,
            repo_url: "https://example.com".to_string(),
            badges: Some("Rust, WebAssembly, egui".to_string()),
            updated_at: None,
        };
        assert!(repo.matches_query("webassembly"));
        assert!(repo.matches_query("egui"));
        assert!(!repo.matches_query("python"));
    }

    #[test]
    fn matches_query_empty_returns_true() {
        let repo = RepoSummary {
            name: "Example".to_string(),
            description: "Description".to_string(),
            image_url: None,
            repo_url: "https://example.com".to_string(),
            badges: None,
            updated_at: None,
        };
        assert!(repo.matches_query(""));
    }

    #[test]
    fn filter_by_query_filters_section_items() {
        let section = RepoSection {
            name: "Test Section".to_string(),
            meta: SectionMeta::default(),
            items: vec![
                RepoSummary {
                    name: "Rust Project".to_string(),
                    description: "A Rust library".to_string(),
                    image_url: None,
                    repo_url: "https://example.com/rust".to_string(),
                    badges: Some("rust, wasm".to_string()),
                    updated_at: None,
                },
                RepoSummary {
                    name: "Python Project".to_string(),
                    description: "A Python tool".to_string(),
                    image_url: None,
                    repo_url: "https://example.com/python".to_string(),
                    badges: Some("python".to_string()),
                    updated_at: None,
                },
            ],
        };

        let filtered = section.filter_by_query("rust");
        assert_eq!(filtered.items.len(), 1);
        assert_eq!(filtered.items[0].name, "Rust Project");

        let filtered_all = section.filter_by_query("");
        assert_eq!(filtered_all.items.len(), 2);
    }
}
