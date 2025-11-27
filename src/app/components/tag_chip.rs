use std::borrow::Cow;

const HERO_TAG_COLOR: egui::Color32 = egui::Color32::from_rgb(59, 154, 255);

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
            .fill(egui::Color32::from_black_alpha(100))
            .stroke(egui::Stroke::new(1.0, HERO_TAG_COLOR))
            .corner_radius(10.0)
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(self.text.as_ref())
                        .color(egui::Color32::from_white_alpha(220))
                        .small(),
                );
            });
    }
}
