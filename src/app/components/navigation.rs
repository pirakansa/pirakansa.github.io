use super::settings_menu::{SettingsAction, SettingsMenu};
use crate::app::layout::ResponsiveLayout;
use crate::app::theme::{background, text};

const PRIMARY_NAV_ITEMS: &[&str] = &["ホーム"];
const SECONDARY_NAV_ITEMS: &[&str] = &["マイリスト"];

/// Renders the shared top navigation bar with search and profile shortcuts.
pub(crate) struct NavigationBar<'a> {
    search_query: &'a mut String,
    settings_menu: SettingsMenu<'a>,
    layout: ResponsiveLayout,
}

impl<'a> NavigationBar<'a> {
    pub(crate) fn new(
        search_query: &'a mut String,
        settings_menu_open: &'a mut bool,
        layout: ResponsiveLayout,
    ) -> Self {
        Self {
            search_query,
            settings_menu: SettingsMenu::new(settings_menu_open),
            layout,
        }
    }

    pub(crate) fn show(self, ui: &mut egui::Ui) -> Option<SettingsAction> {
        let NavigationBar {
            search_query,
            mut settings_menu,
            layout,
        } = self;

        egui::Frame::default()
            .fill(background::NAV_BAR)
            .inner_margin(egui::Margin::symmetric(16, 12))
            .corner_radius(12.0)
            .show(ui, |ui| {
                if layout.is_compact() {
                    Self::compact(ui, search_query, &mut settings_menu, layout)
                } else {
                    Self::spacious(ui, search_query, &mut settings_menu, layout)
                }
            })
            .inner
    }

    fn compact(
        ui: &mut egui::Ui,
        search_query: &mut String,
        settings_menu: &mut SettingsMenu<'_>,
        layout: ResponsiveLayout,
    ) -> Option<SettingsAction> {
        ui.vertical(|ui| {
            let mut action = None;
            ui.spacing_mut().item_spacing.y = 8.0;
            ui.horizontal(|ui| {
                title_label(ui);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    action = settings_menu.show(ui);
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("🔔").color(text::PRIMARY));
                });
            });
            search_field(ui, search_query, layout);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 14.0;
                for item in PRIMARY_NAV_ITEMS {
                    ui.label(egui::RichText::new(*item).strong().color(text::PRIMARY));
                }
                for item in SECONDARY_NAV_ITEMS {
                    ui.label(egui::RichText::new(*item).color(text::SECONDARY));
                }
            });
            action
        })
        .inner
    }

    fn spacious(
        ui: &mut egui::Ui,
        search_query: &mut String,
        settings_menu: &mut SettingsMenu<'_>,
        layout: ResponsiveLayout,
    ) -> Option<SettingsAction> {
        ui.horizontal(|ui| {
            let mut action = None;
            ui.horizontal(|ui| {
                title_label(ui);
                ui.add_space(20.0);
                for item in PRIMARY_NAV_ITEMS {
                    ui.label(egui::RichText::new(*item).strong().color(text::PRIMARY));
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                action = settings_menu.show(ui);
                ui.add_space(10.0);
                ui.label(egui::RichText::new("🔔").color(text::PRIMARY));
                ui.add_space(16.0);
                search_field(ui, search_query, layout);
                ui.add_space(14.0);
                for item in SECONDARY_NAV_ITEMS.iter().rev() {
                    ui.label(egui::RichText::new(*item).color(text::SECONDARY));
                }
            });
            action
        })
        .inner
    }
}

fn title_label(ui: &mut egui::Ui) {
    ui.label(
        egui::RichText::new("Repositories Map")
            .heading()
            .color(text::ACCENT),
    );
}

fn search_field(ui: &mut egui::Ui, search_query: &mut String, layout: ResponsiveLayout) {
    let width = layout.search_width();
    egui::Frame::new()
        .fill(background::SEARCH_FIELD)
        .corner_radius(6.0)
        .inner_margin(egui::Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.add_sized(
                [width - 16.0, if layout.is_compact() { 24.0 } else { 22.0 }],
                egui::TextEdit::singleline(search_query)
                    .frame(false)
                    .hint_text("リポジトリ名、技術スタック、キーワードを検索")
                    .text_color(text::DARK),
            );
        });
}
