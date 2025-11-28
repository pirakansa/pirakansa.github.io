use crate::app::data::RepoSummary;
use crate::app::layout::ResponsiveLayout;

use super::tag_chip::TagChip;

const CARD_STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(38, 45, 66);

/// Shows a single repository summary as a compact card.
pub(crate) struct RepoCard<'a> {
    repo: &'a RepoSummary,
    layout: ResponsiveLayout,
}

impl<'a> RepoCard<'a> {
    pub(crate) fn new(repo: &'a RepoSummary, layout: ResponsiveLayout) -> Self {
        Self { repo, layout }
    }

    pub(crate) fn show(self, ui: &mut egui::Ui) {
        let RepoCard { repo, layout } = self;
        let card_size = egui::vec2(layout.card_width(), 300.0);
        ui.allocate_ui_with_layout(card_size, egui::Layout::top_down(egui::Align::Min), |ui| {
            egui::Frame::default()
                .fill(egui::Color32::from_rgb(26, 28, 38))
                .stroke(egui::Stroke::new(1.0, CARD_STROKE_COLOR))
                .corner_radius(14.0)
                .inner_margin(egui::Margin::symmetric(16, 12))
                .show(ui, |ui| {
                    ui.set_width(card_size.x);
                    ui.set_height(card_size.y);
                    ui.vertical(|ui| {
                        egui::Frame::default()
                            .fill(egui::Color32::from_rgb(58, 90, 158))
                            .corner_radius(12.0)
                            .show(ui, |ui| {
                                ui.set_height(layout.preview_height());
                                ui.centered_and_justified(|ui| {
                                    if let Some(image_url) = &repo.image_url {
                                        let max_size = egui::vec2(
                                            ui.available_width(),
                                            layout.preview_height(),
                                        );
                                        let image = egui::Image::from_uri(image_url.clone())
                                            .maintain_aspect_ratio(true)
                                            .max_size(max_size)
                                            .shrink_to_fit()
                                            .corner_radius(10.0);
                                        ui.add(image);
                                    } else {
                                        ui.label(
                                            egui::RichText::new("リポジトリアイコン")
                                                .color(egui::Color32::from_white_alpha(180))
                                                .small(),
                                        );
                                    }
                                });
                            });

                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(&repo.name)
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        ui.label(
                            egui::RichText::new(&repo.description)
                                .small()
                                .color(egui::Color32::from_gray(200)),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(format!("スター: {}", repo.stars))
                                .small()
                                .color(egui::Color32::WHITE),
                        );
                        if let Some(badge) = &repo.badge {
                            ui.add_space(4.0);
                            TagChip::new(badge).show(ui);
                        }
                    });
                });
        });
    }
}
