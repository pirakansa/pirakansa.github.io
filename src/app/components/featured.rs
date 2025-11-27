use crate::app::data::FeaturedRepo;
use crate::app::layout::ResponsiveLayout;

use super::tag_chip::TagChip;
use image::load_from_memory;
use std::sync::OnceLock;

/// Draws the hero section that highlights the featured repository.
pub(crate) struct FeaturedSection<'a> {
    featured: &'a FeaturedRepo,
    layout: ResponsiveLayout,
}

impl<'a> FeaturedSection<'a> {
    pub(crate) fn new(featured: &'a FeaturedRepo, layout: ResponsiveLayout) -> Self {
        Self { featured, layout }
    }

    pub(crate) fn show(self, ui: &mut egui::Ui) {
        let FeaturedSection { featured, layout } = self;
        egui::Frame::default()
            .fill(egui::Color32::from_rgb(18, 24, 39))
            .shadow(egui::epaint::Shadow {
                offset: [0, 8],
                blur: 24,
                spread: 0,
                color: egui::Color32::from_black_alpha(80),
            })
            .corner_radius(20.0)
            .inner_margin(egui::Margin::same(24))
            .show(ui, |ui| {
                if layout.is_compact() {
                    Self::hero_copy(ui, featured, layout);
                    ui.add_space(18.0);
                    Self::hero_image(ui, layout);
                } else {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            Self::hero_copy(ui, featured, layout);
                        });
                        ui.add_space(24.0);
                        ui.vertical(|ui| {
                            Self::hero_image(ui, layout);
                        });
                    });
                }
            });
    }

    fn hero_copy(ui: &mut egui::Ui, featured: &FeaturedRepo, layout: ResponsiveLayout) {
        let title_size = if layout.is_compact() { 28.0 } else { 32.0 };
        ui.label(
            egui::RichText::new(&featured.name)
                .size(title_size)
                .strong()
                .color(egui::Color32::from_white_alpha(235)),
        );
        ui.add_space(8.0);
        ui.label(egui::RichText::new(&featured.description).color(egui::Color32::from_gray(210)));
        ui.add_space(12.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for tag in &featured.tags {
                TagChip::new(tag).show(ui);
            }
            let star_text = format!("⭐ {}", featured.stars);
            TagChip::new(star_text).show(ui);
        });
        ui.add_space(18.0);
        let repo_button = || {
            egui::Button::new(
                egui::RichText::new("🔗 リポジトリを開く").color(egui::Color32::BLACK),
            )
            .fill(egui::Color32::from_rgb(255, 255, 255))
            .min_size(egui::vec2(140.0, 40.0))
            .corner_radius(10.0)
        };
        let detail_button = || {
            egui::Button::new("README を見る")
                .fill(egui::Color32::TRANSPARENT)
                .stroke(egui::Stroke {
                    width: 1.0,
                    color: egui::Color32::from_white_alpha(180),
                })
                .corner_radius(10.0)
        };
        if layout.is_phone() {
            ui.vertical(|ui| {
                ui.add(repo_button());
                ui.add_space(8.0);
                ui.add(detail_button());
            });
        } else {
            ui.horizontal(|ui| {
                ui.add(repo_button());
                ui.add(detail_button());
            });
        }
    }

    fn hero_image(ui: &mut egui::Ui, layout: ResponsiveLayout) {
        let max_width = if layout.is_compact() {
            ui.available_width()
        } else {
            320.0
        };
        let max_height = if layout.is_phone() { 170.0 } else { 220.0 };
        // ui.centered_and_justified(|ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            let color_image = Self::logo_image();
            let texture = ui.ctx().load_texture(
                "featured-logo",
                color_image.clone(),
                egui::TextureOptions::LINEAR,
            );
            let [width, height] = color_image.size;
            let sized_texture = egui::load::SizedTexture::new(
                texture.id(),
                egui::vec2(width as f32, height as f32),
            );
            let image_widget = egui::Image::new(sized_texture)
                .maintain_aspect_ratio(true)
                .max_size(egui::vec2(max_width, max_height))
                .shrink_to_fit()
                .corner_radius(12.0);
            ui.add(image_widget);
        });
    }

    fn logo_image() -> &'static egui::ColorImage {
        static LOGO_IMAGE: OnceLock<egui::ColorImage> = OnceLock::new();
        LOGO_IMAGE.get_or_init(|| {
            let rgba = load_from_memory(include_bytes!("../../../assets/logo.jpeg"))
                .expect("embedded logo file should decode")
                .to_rgba8();
            let (width, height) = rgba.dimensions();
            egui::ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                rgba.as_raw(),
            )
        })
    }
}
