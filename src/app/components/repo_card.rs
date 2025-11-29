use crate::app::data::RepoSummary;
use crate::app::layout::ResponsiveLayout;
use crate::app::theme::{background, stroke, text};

use super::tag_chip::TagChip;

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
                .fill(background::CARD)
                .stroke(egui::Stroke::new(1.0, stroke::CARD))
                .corner_radius(14.0)
                .inner_margin(egui::Margin::symmetric(16, 12))
                .show(ui, |ui| {
                    ui.set_width(card_size.x);
                    ui.set_height(card_size.y);
                    ui.vertical(|ui| {
                        egui::Frame::default()
                            .fill(background::CARD_PREVIEW)
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
                                        let fallback_icon = egui::RichText::new("🖼")
                                            .color(text::WHITE_ALPHA_180)
                                            .size(layout.preview_height() * 0.5);
                                        ui.label(fallback_icon);
                                    }
                                });
                            });

                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(&repo.name)
                                .strong()
                                .color(text::PRIMARY),
                        );
                        ui.label(
                            egui::RichText::new(&repo.description)
                                .small()
                                .color(text::SECONDARY),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(format!("最終更新: {}", repo.updated_at))
                                .small()
                                .color(text::PRIMARY),
                        );
                        if let Some(badges) = &repo.badges {
                            ui.add_space(4.0);
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing.x = 4.0;
                                for badge in badges
                                    .split(',')
                                    .map(|s| s.trim())
                                    .filter(|s| !s.is_empty())
                                {
                                    TagChip::new(badge).show(ui);
                                }
                            });
                        }
                    });
                });
        });
    }
}
