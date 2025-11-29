#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub(crate) struct RepoSummary {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) image_url: Option<String>,
    #[serde(default)]
    pub(crate) badges: Option<String>,
    pub(crate) updated_at: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub(crate) struct RepoSection {
    pub(crate) name: String,
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

pub(crate) fn load_portfolio_data() -> Vec<RepoSection> {
    const PORTFOLIO_YAML: &str = include_str!("../../assets/portfolio.yaml");

    serde_yaml::from_str::<Vec<RepoSection>>(PORTFOLIO_YAML)
        .expect("portfolio YAML should be valid")
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
    fn load_portfolio_data_reads_sections() {
        let sections = load_portfolio_data();
        assert!(
            !sections.is_empty(),
            "sections should still load from portfolio.yaml"
        );
    }
}
