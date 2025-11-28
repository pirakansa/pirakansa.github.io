use std::borrow::Cow;

use crate::app::theme::{background, text};

/// Compact chip-like label for tags or metadata such as star counts.
pub(crate) struct TagChip<'a> {
    text: Cow<'a, str>,
}

impl<'a> TagChip<'a> {
    pub(crate) fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self { text: text.into() }
    }

    pub(crate) fn show(self, ui: &mut egui::Ui) {
        egui::Frame::default()
            .fill(background::TAG_CHIP)
            .stroke(egui::Stroke::new(1.0, text::ACCENT))
            .corner_radius(10.0)
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(self.text.as_ref())
                        .color(text::WHITE_ALPHA_180)
                        .small(),
                );
            });
    }
}
