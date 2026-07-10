use crate::length::Length;

/// Represents a rectangular area on the screen.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rect {
    top: Length,
    bottom: Length,
    left: Length,
    right: Length,
}
impl Rect {
    /// Constructs a Rect from given side positions.
    pub fn new(top: Length, bottom: Length, left: Length, right: Length) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    // Constructs a Rect from given position and size
    // Note that this might give less flexibility than creating points manually.
    // This function will return None if all parameters aren't the same length unit.
    pub fn from_pos_size(x: Length, y: Length, width: Length, height: Length) -> Option<Self> {
        let top = match (y, height) {
            (Length::Pixels(a), Length::Pixels(b)) => Length::Pixels(a - b / 2),
            (Length::Perc(a), Length::Perc(b)) => Length::Perc(a - b / 2.0),
            _ => {
                return None;
            }
        };

        let bottom = match (y, height) {
            (Length::Pixels(a), Length::Pixels(b)) => Length::Pixels(a + b / 2),
            (Length::Perc(a), Length::Perc(b)) => Length::Perc(a + b / 2.0),
            _ => {
                return None;
            }
        };

        let left = match (x, width) {
            (Length::Pixels(a), Length::Pixels(b)) => Length::Pixels(a - b / 2),
            (Length::Perc(a), Length::Perc(b)) => Length::Perc(a - b / 2.0),
            _ => {
                return None;
            }
        };

        let right = match (x, width) {
            (Length::Pixels(a), Length::Pixels(b)) => Length::Pixels(a + b / 2),
            (Length::Perc(a), Length::Perc(b)) => Length::Perc(a + b / 2.0),
            _ => {
                return None;
            }
        };

        Some(Self {
            top,
            bottom,
            left,
            right,
        })
    }

    /// Returns the Rect's middle position in pixels.
    pub fn get_pos_pixels(&self, ref_width: f32, ref_height: f32) -> (Length, Length) {
        let top = match self.top {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let bottom = match self.bottom {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let left = match self.left {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let right = match self.right {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let y = (top + bottom).abs() / 2;
        let x = (left + right).abs() / 2;

        (Length::Pixels(x), Length::Pixels(y))
    }

    /// Returns the Rect's middle position in percentages.
    pub fn get_pos_perc(&self, ref_width: f32, ref_height: f32) -> (Length, Length) {
        let (x, y) = self.get_pos_pixels(ref_width, ref_height);

        (x.to_perc(ref_width), y.to_perc(ref_height))
    }

    /// Returns the Rect's full extents in pixels.
    pub fn get_size_pixels(&self, ref_width: f32, ref_height: f32) -> (Length, Length) {
        let top = match self.top {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let bottom = match self.bottom {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let left = match self.left {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let right = match self.right {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let y = (top - bottom).abs();
        let x = (left - right).abs();

        (Length::Pixels(x), Length::Pixels(y))
    }

    /// Returns the Rect's full extents in percentages.
    pub fn get_size_perc(&self, ref_width: f32, ref_height: f32) -> (Length, Length) {
        let (x, y) = self.get_pos_pixels(ref_width, ref_height);

        (x.to_perc(ref_width), y.to_perc(ref_height))
    }

    /// Returns true if the given point is inside the Rect.
    pub fn contains_point(&self, x: i32, y: i32, ref_width: f32, ref_height: f32) -> bool {
        let top = match self.top {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let bottom = match self.bottom {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let left = match self.left {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let right = match self.right {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        x >= left && x <= right && y >= top && y <= bottom
    }

    /// Returns true if two Rects overlap each other.
    pub fn overlaps(&self, other: &Rect, ref_width: f32, ref_height: f32) -> bool {
        let top1 = match self.top {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let bottom1 = match self.bottom {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let left1 = match self.left {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let right1 = match self.right {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let top2 = match other.top {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let bottom2 = match other.bottom {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let left2 = match other.left {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let right2 = match other.right {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let x_check = right1 >= left2 || left1 <= right2;
        let y_check = bottom1 >= top2 || top1 <= bottom2;

        x_check && y_check
    }

    /// Returns true if the other Rect is inside this one.
    pub fn contains_rect(&self, other: &Rect, ref_width: f32, ref_height: f32) -> bool {
        let top1 = match self.top {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let bottom1 = match self.bottom {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let left1 = match self.left {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let right1 = match self.right {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let top2 = match other.top {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let bottom2 = match other.bottom {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_height) as i32,
        };

        let left2 = match other.left {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let right2 = match other.right {
            Length::Pixels(x) => x,
            Length::Perc(x) => (x * ref_width) as i32,
        };

        let x_check = right1 >= left2 && left1 <= right2;
        let y_check = bottom1 >= top2 && top1 <= bottom2;

        x_check && y_check
    }
}
