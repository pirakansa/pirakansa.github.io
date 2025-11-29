/// Footer widget that houses the egui/eframe attribution links.
pub(crate) struct AttributionFooter;

impl AttributionFooter {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn show(self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Powered by ");
            ui.hyperlink_to("pirakansa", "https://github.com/pirakansa");
            ui.label(".");
            ui.separator();
            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
        });
    }
}
