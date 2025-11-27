use crate::app::layout::ResponsiveLayout;

const PRIMARY_NAV_ITEMS: &[&str] = &["ホーム"];
const SECONDARY_NAV_ITEMS: &[&str] = &["マイリスト"];

/// Renders the shared top navigation bar with search and profile shortcuts.
pub(crate) struct NavigationBar<'a> {
    search_query: &'a mut String,
    layout: ResponsiveLayout,
}

impl<'a> NavigationBar<'a> {
    pub(crate) fn new(search_query: &'a mut String, layout: ResponsiveLayout) -> Self {
        Self {
            search_query,
            layout,
        }
    }

    pub(crate) fn show(self, ui: &mut egui::Ui) {
        let NavigationBar {
            search_query,
            layout,
        } = self;

        egui::Frame::default()
            .fill(egui::Color32::from_rgba_premultiplied(5, 7, 16, 230))
            .inner_margin(egui::Margin::symmetric(16, 12))
            .corner_radius(12.0)
            .show(ui, |ui| {
                if layout.is_compact() {
                    Self::compact(ui, search_query, layout);
                } else {
                    Self::spacious(ui, search_query, layout);
                }
            });
    }

    fn compact(ui: &mut egui::Ui, search_query: &mut String, layout: ResponsiveLayout) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 8.0;
            ui.horizontal(|ui| {
                title_label(ui);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("👤 プロフィール"));
                    ui.add_space(10.0);
                    ui.label("🔔");
                });
            });
            search_field(ui, search_query, layout);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 14.0;
                for item in PRIMARY_NAV_ITEMS {
                    ui.label(egui::RichText::new(*item).strong());
                }
                for item in SECONDARY_NAV_ITEMS {
                    ui.label(*item);
                }
            });
        });
    }

    fn spacious(ui: &mut egui::Ui, search_query: &mut String, layout: ResponsiveLayout) {
        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                title_label(ui);
                ui.add_space(20.0);
                for item in PRIMARY_NAV_ITEMS {
                    ui.label(egui::RichText::new(*item).strong());
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new("👤 プロフィール"));
                ui.add_space(10.0);
                ui.label("🔔");
                ui.add_space(16.0);
                search_field(ui, search_query, layout);
                ui.add_space(14.0);
                for item in SECONDARY_NAV_ITEMS.iter().rev() {
                    ui.label(*item);
                }
            });
        });
    }
}

fn title_label(ui: &mut egui::Ui) {
    ui.label(
        egui::RichText::new("Repositories Map")
            .heading()
            .color(egui::Color32::from_rgb(59, 154, 255)),
    );
}

fn search_field(ui: &mut egui::Ui, search_query: &mut String, layout: ResponsiveLayout) {
    let width = layout.search_width();
    ui.add_sized(
        [width, if layout.is_compact() { 32.0 } else { 30.0 }],
        egui::TextEdit::singleline(search_query)
            .hint_text("リポジトリ名、技術スタック、キーワードを検索"),
    );
}
