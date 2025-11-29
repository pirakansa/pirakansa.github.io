use crate::app::theme::{background, button, stroke, text};

/// Handles the settings dropdown menu shown from the navigation bar.
pub(crate) struct SettingsMenu<'a> {
    open: &'a mut bool,
}

impl<'a> SettingsMenu<'a> {
    pub(crate) fn new(open: &'a mut bool) -> Self {
        Self { open }
    }

    pub(crate) fn show(&mut self, ui: &mut egui::Ui) {
        let response = ui.add(Self::button());
        let mut just_opened = false;

        if response.clicked() {
            if *self.open {
                *self.open = false;
            } else {
                *self.open = true;
                just_opened = true;
            }
        }

        self.show_popup(ui.ctx(), response.rect, just_opened);
    }

    fn button() -> egui::Button<'static> {
        egui::Button::new(egui::RichText::new("⚙ 設定").color(text::PRIMARY))
            .fill(button::settings_fill())
            .min_size(egui::vec2(90.0, 32.0))
            .corner_radius(8.0)
    }

    fn show_popup(&mut self, ctx: &egui::Context, button_rect: egui::Rect, just_opened: bool) {
        if !*self.open {
            return;
        }

        let menu_width = 200.0;
        let anchor_x = (button_rect.right() - menu_width).max(8.0);
        let anchor = egui::pos2(anchor_x, button_rect.bottom() + 8.0);
        let popup_area = egui::Area::new(egui::Id::new("settings-menu"))
            .order(egui::Order::Foreground)
            .fixed_pos(anchor);

        let popup = popup_area.show(ctx, |ui| {
            egui::Frame::popup(ui.style())
                .fill(background::CARD)
                .stroke(egui::Stroke::new(1.0, stroke::CARD))
                .corner_radius(10.0)
                .inner_margin(egui::Margin::symmetric(12, 8))
                .show(ui, |ui| {
                    ui.set_width(menu_width);
                    let mut should_close = false;

                    if menu_item(ui, "表示設定 (準備中)").clicked() {
                        should_close = true;
                    }

                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION")))
                            .color(text::SECONDARY)
                            .size(12.0),
                    );

                    should_close
                })
                .inner
        });

        let menu_rect = popup.response.rect;
        let close_on_escape = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        let clicked_outside = ctx.input(|i| {
            i.pointer.any_pressed()
                && i.pointer
                    .interact_pos()
                    .is_some_and(|pos| !menu_rect.contains(pos) && !button_rect.contains(pos))
        });

        if popup.inner || close_on_escape || (!just_opened && clicked_outside) {
            *self.open = false;
        }
    }
}

fn menu_item(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(text::PRIMARY))
            .fill(background::CARD_PREVIEW)
            .min_size(egui::vec2(ui.available_width(), 30.0))
            .frame(false)
            .corner_radius(6.0),
    )
}
