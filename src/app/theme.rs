//! Centralized theme colors to ensure consistency across WASM and native builds.
//!
//! All UI colors should be defined here to avoid platform-specific rendering differences.

use egui::Color32;

/// Text colors
pub mod text {
    use super::*;

    /// Primary text color (white)
    pub const PRIMARY: Color32 = Color32::WHITE;

    /// Secondary text color (light gray)
    pub const SECONDARY: Color32 = Color32::from_rgb(200, 200, 200);

    /// Muted text color (darker gray)
    pub const MUTED: Color32 = Color32::from_rgb(210, 210, 210);

    /// Dark text color (for light backgrounds)
    pub const DARK: Color32 = Color32::from_rgb(30, 30, 35);

    /// Accent text color (blue)
    pub const ACCENT: Color32 = Color32::from_rgb(59, 154, 255);

    /// Semi-transparent white text
    pub const WHITE_ALPHA_235: Color32 = Color32::from_rgba_premultiplied(235, 235, 235, 235);

    /// Semi-transparent white text (lighter)
    pub const WHITE_ALPHA_180: Color32 = Color32::from_rgba_premultiplied(180, 180, 180, 180);
}

/// Background colors
pub mod background {
    use super::*;

    /// Main app background
    pub const APP: Color32 = Color32::from_rgb(9, 11, 19);

    /// Navigation bar background
    pub const NAV_BAR: Color32 = Color32::from_rgba_premultiplied(5, 7, 16, 230);

    /// Featured section background
    pub const FEATURED: Color32 = Color32::from_rgb(18, 24, 39);

    /// Card background
    pub const CARD: Color32 = Color32::from_rgb(26, 28, 38);

    /// Card preview placeholder background
    pub const CARD_PREVIEW: Color32 = Color32::from_rgb(35, 40, 55);

    /// Search field background
    pub const SEARCH_FIELD: Color32 = Color32::from_rgb(240, 240, 245);

    /// Tag chip background
    pub const TAG_CHIP: Color32 = Color32::from_rgba_premultiplied(100, 100, 100, 100);
}

/// Stroke/border colors
pub mod stroke {
    use super::*;

    /// Card border color
    pub const CARD: Color32 = Color32::from_rgb(38, 45, 66);

    /// Button border (semi-transparent white)
    pub const BUTTON: Color32 = Color32::from_rgba_premultiplied(180, 180, 180, 180);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure all text colors are visible (non-zero alpha)
    #[test]
    fn text_colors_are_visible() {
        assert!(text::PRIMARY.a() > 0, "PRIMARY text should be visible");
        assert!(text::SECONDARY.a() > 0, "SECONDARY text should be visible");
        assert!(text::MUTED.a() > 0, "MUTED text should be visible");
        assert!(text::DARK.a() > 0, "DARK text should be visible");
        assert!(text::ACCENT.a() > 0, "ACCENT text should be visible");
        assert!(
            text::WHITE_ALPHA_235.a() > 0,
            "WHITE_ALPHA_235 text should be visible"
        );
        assert!(
            text::WHITE_ALPHA_180.a() > 0,
            "WHITE_ALPHA_180 text should be visible"
        );
    }

    /// Ensure text colors have sufficient contrast for readability
    #[test]
    fn primary_text_is_bright_enough() {
        // PRIMARY should be close to white (high RGB values)
        assert!(
            text::PRIMARY.r() >= 200,
            "PRIMARY text red channel should be bright"
        );
        assert!(
            text::PRIMARY.g() >= 200,
            "PRIMARY text green channel should be bright"
        );
        assert!(
            text::PRIMARY.b() >= 200,
            "PRIMARY text blue channel should be bright"
        );
    }

    /// Ensure dark text is actually dark (for light backgrounds)
    #[test]
    fn dark_text_is_dark_enough() {
        assert!(text::DARK.r() <= 50, "DARK text red channel should be dark");
        assert!(
            text::DARK.g() <= 50,
            "DARK text green channel should be dark"
        );
        assert!(
            text::DARK.b() <= 50,
            "DARK text blue channel should be dark"
        );
    }

    /// Ensure search field background is light enough for dark text
    #[test]
    fn search_field_background_is_light() {
        assert!(
            background::SEARCH_FIELD.r() >= 200,
            "SEARCH_FIELD background should be light"
        );
        assert!(
            background::SEARCH_FIELD.g() >= 200,
            "SEARCH_FIELD background should be light"
        );
        assert!(
            background::SEARCH_FIELD.b() >= 200,
            "SEARCH_FIELD background should be light"
        );
    }
}
