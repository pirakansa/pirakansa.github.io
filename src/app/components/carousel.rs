use crate::app::data::RepoSection;
use crate::app::layout::ResponsiveLayout;
use crate::app::theme::text;

use super::repo_card::RepoCard;

/// Displays each repository section as a horizontal scrolling carousel.
pub(crate) struct RepoCarousel<'a> {
    section: &'a RepoSection,
    layout: ResponsiveLayout,
}

impl<'a> RepoCarousel<'a> {
    pub(crate) fn new(section: &'a RepoSection, layout: ResponsiveLayout) -> Self {
        Self { section, layout }
    }

    pub(crate) fn show(self, ui: &mut egui::Ui) {
        let RepoCarousel { section, layout } = self;
        ui.heading(
            egui::RichText::new(&section.name)
                .size(18.0)
                .color(text::SECONDARY),
        );
        egui::ScrollArea::horizontal()
            .id_salt(section.name.as_str())
            .animated(true)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    for repo in &section.repos {
                        ui.push_id((&section.name, &repo.name), |ui| {
                            RepoCard::new(repo, layout).show(ui);
                        });
                        ui.add_space(12.0);
                    }
                });
                // Keep the scroll bar from overlapping the card content.
                ui.add_space(8.0);
            });
    }
}
