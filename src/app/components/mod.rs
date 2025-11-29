//! Reusable building blocks that compose the UI layer.
mod carousel;
mod featured;
mod footer;
mod navigation;
mod repo_card;
mod settings_menu;
mod tag_chip;

pub(crate) use carousel::RepoCarousel;
pub(crate) use featured::FeaturedSection;
pub(crate) use footer::AttributionFooter;
pub(crate) use navigation::NavigationBar;
pub(crate) use settings_menu::SettingsAction;
