#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum HAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl HAlign {
    pub fn is_left(&self) -> bool {
        matches!(self, HAlign::Left)
    }

    pub fn is_center(&self) -> bool {
        matches!(self, HAlign::Center)
    }

    pub fn is_right(&self) -> bool {
        matches!(self, HAlign::Right)
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum VAlign {
    #[default]
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Align(HAlign, VAlign);
