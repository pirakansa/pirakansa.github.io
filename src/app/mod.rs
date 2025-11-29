mod components;
mod data;
mod fonts;
mod layout;
pub mod theme;

use components::{AttributionFooter, FeaturedSection, NavigationBar, RepoCarousel};
use data::{load_featured_repo, FeaturedRepo, PortfolioLoadState, PortfolioLoader, RepoSection};
use egui_extras::install_image_loaders;
use fonts::install_fonts;
use layout::ResponsiveLayout;
use theme::background;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    featured: FeaturedRepo,
    sections: Vec<RepoSection>,
    search_query: String,
    #[serde(skip)]
    portfolio_loader: PortfolioLoader,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let featured = load_featured_repo();

        Self {
            featured,
            sections: Vec::new(),
            search_query: String::new(),
            portfolio_loader: PortfolioLoader::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        install_fonts(&cc.egui_ctx);
        install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        // Debug builds skip persistence to make data edits reflect immediately.
        if cfg!(debug_assertions) {
            return TemplateApp::default();
        }

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // URLからポートフォリオデータの取得を開始
        self.portfolio_loader.start_loading(ctx);

        // ロード完了時にデータを更新
        if let PortfolioLoadState::Loaded(sections) = self.portfolio_loader.state() {
            self.sections = sections;
        }

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(background::APP)
                    .inner_margin(egui::Margin::symmetric(20, 16)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("main-scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(18.0, 14.0);
                        let layout = ResponsiveLayout::from_width(ui.available_width());
                        NavigationBar::new(&mut self.search_query, layout).show(ui);
                        FeaturedSection::new(&self.featured, layout).show(ui);
                        ui.separator();

                        // ポートフォリオの状態に応じて表示を変更
                        match self.portfolio_loader.state() {
                            PortfolioLoadState::Loading | PortfolioLoadState::NotStarted => {
                                ui.spinner();
                                ui.label("Loading portfolio...");
                            }
                            PortfolioLoadState::Error(err) => {
                                ui.colored_label(egui::Color32::RED, format!("Error: {err}"));
                                // エラー時はフォールバックデータを表示
                                for section in &self.sections {
                                    RepoCarousel::new(section, layout).show(ui);
                                }
                            }
                            PortfolioLoadState::Loaded(_) => {
                                for section in &self.sections {
                                    RepoCarousel::new(section, layout).show(ui);
                                }
                            }
                        }

                        ui.add_space(12.0);
                        AttributionFooter::new().show(ui);
                    });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::fonts::{noto_sans_font_definitions, NOTO_SANS_JP_FONT_ID};
    use super::*;

    #[test]
    fn default_ui_data_contains_featured() {
        let app = TemplateApp::default();

        assert!(!app.featured.name.is_empty());
        // sections は URL から非同期でロードされるため、初期状態では空
        assert!(app.sections.is_empty(), "セクションは初期状態で空");
    }

    #[test]
    fn noto_sans_font_is_first_in_all_families() {
        let fonts = noto_sans_font_definitions();

        assert!(
            fonts.font_data.contains_key(NOTO_SANS_JP_FONT_ID),
            "font data should include bundled Japanese font"
        );

        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            let entries = fonts
                .families
                .get(&family)
                .expect("font family should exist");
            assert_eq!(
                entries.first().map(|s| s.as_str()),
                Some(NOTO_SANS_JP_FONT_ID),
                "{family:?} should prioritize the bundled font"
            );
        }
    }
}
