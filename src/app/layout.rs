#[derive(Clone, Copy)]
pub(crate) struct ResponsiveLayout {
    width: f32,
}

impl ResponsiveLayout {
    pub(crate) fn from_width(width: f32) -> Self {
        Self { width }
    }

    pub(crate) fn is_compact(&self) -> bool {
        self.width < 720.0
    }

    pub(crate) fn is_phone(&self) -> bool {
        self.width < 520.0
    }

    pub(crate) fn search_width(&self) -> f32 {
        if self.is_phone() {
            (self.width - 40.0).max(140.0)
        } else if self.is_compact() {
            200.0
        } else {
            240.0
        }
    }

    pub(crate) fn card_width(&self) -> f32 {
        200.0
    }

    pub(crate) fn preview_height(&self) -> f32 {
        150.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_phone_and_desktop_breakpoints() {
        let phone = ResponsiveLayout::from_width(480.0);
        assert!(phone.is_phone());
        assert!(phone.is_compact());

        let desktop = ResponsiveLayout::from_width(960.0);
        assert!(!desktop.is_phone());
        assert!(!desktop.is_compact());
    }

    #[test]
    fn search_field_has_minimum_width_on_tiny_phone() {
        let narrow = ResponsiveLayout::from_width(150.0);
        assert_eq!(narrow.search_width(), 140.0);
    }
}
