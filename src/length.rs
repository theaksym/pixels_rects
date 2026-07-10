use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

/// Used to represent screen space lengths.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Length {
    Pixels(i32),
    Perc(f32),
}
impl Length {
    /// Converts this Length of any variant to a Length::Pixels
    pub fn to_pixels(self, ref_scale: f32) -> Length {
        match self {
            Length::Pixels(x) => Length::Pixels(x),
            Length::Perc(x) => Length::Pixels((x * ref_scale) as i32),
        }
    }

    /// Converts this Length of any variant to a Length::Perc
    pub fn to_perc(self, ref_scale: f32) -> Length {
        match self {
            Length::Pixels(x) => Length::Perc(x as f32 / ref_scale),
            Length::Perc(x) => Length::Perc(x),
        }
    }

    /// Converts this Length of any variant to a i32
    pub fn to_i32(self, ref_scale: f32) -> i32 {
        match self.to_pixels(ref_scale) {
            Length::Pixels(x) => x,
            _ => unreachable!(),
        }
    }
}

/// Converts two Lengths of any variant to two Length::Pixels
pub fn to_pixels(x: Length, y: Length, ref_width: f32, ref_height: f32) -> (Length, Length) {
    (x.to_pixels(ref_width), y.to_pixels(ref_height))
}

/// Converts two Lengths of any variant to two Length::Perc
pub fn to_perc(x: Length, y: Length, ref_width: f32, ref_height: f32) -> (Length, Length) {
    (x.to_perc(ref_width), y.to_perc(ref_height))
}

/// Converts two Lengths of any variant to two i32s
pub fn to_i32(x: Length, y: Length, ref_width: f32, ref_height: f32) -> (i32, i32) {
    match to_pixels(x, y, ref_width, ref_height) {
        (Length::Pixels(x), Length::Pixels(y)) => (x, y),
        _ => unreachable!(),
    }
}

pub enum LengthOperation {
    Add,
    Sub,
    Mul,
    Div,
}
pub enum LengthOperationError {
    PercPercAddError,
    PercPercSubError,
    PixelPercAddError,
    PixelPercSubError,
    PercPixelAddError,
    PercPixelSubError,
}
impl Display for LengthOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthOperationError::PercPercAddError => f.write_str("PercPercAddError"),
            LengthOperationError::PercPercSubError => f.write_str("PercPercSubError"),
            LengthOperationError::PixelPercAddError => f.write_str("PixelPercAddError"),
            LengthOperationError::PixelPercSubError => f.write_str("PixelPercSubError"),
            LengthOperationError::PercPixelAddError => f.write_str("PercPixelAddError"),
            LengthOperationError::PercPixelSubError => f.write_str("PercPixelSubError"),
        }
    }
}
impl Add for Length {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match length_operation(self, rhs, LengthOperation::Add) {
            Ok(x) => x,
            Err(y) => panic!("Length add operation failed! Error: {}", y),
        }
    }
}
impl Sub for Length {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match length_operation(self, rhs, LengthOperation::Sub) {
            Ok(x) => x,
            Err(y) => panic!("Length sub operation failed! Error: {}", y),
        }
    }
}

impl Mul for Length {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match length_operation(self, rhs, LengthOperation::Mul) {
            Ok(x) => x,
            Err(y) => panic!("Length mul operation failed! Error: {}", y),
        }
    }
}
impl Div for Length {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match length_operation(self, rhs, LengthOperation::Div) {
            Ok(x) => x,
            Err(y) => panic!("Length div operation failed! Error: {}", y),
        }
    }
}
impl AddAssign for Length {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl SubAssign for Length {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl MulAssign for Length {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl DivAssign for Length {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
pub fn length_operation(
    a: Length,
    b: Length,
    op: LengthOperation,
) -> Result<Length, LengthOperationError> {
    match (a, b) {
        (Length::Pixels(x), Length::Pixels(y)) => match op {
            LengthOperation::Add => Ok(Length::Pixels(x + y)),
            LengthOperation::Sub => Ok(Length::Pixels(x - y)),
            LengthOperation::Mul => Ok(Length::Pixels(x * y)),
            LengthOperation::Div => Ok(Length::Pixels((x as f32 / y as f32) as i32)),
        },

        (Length::Perc(x), Length::Perc(y)) => match op {
            LengthOperation::Add => Err(LengthOperationError::PercPercAddError),
            LengthOperation::Sub => Err(LengthOperationError::PercPercSubError),
            LengthOperation::Mul => Ok(Length::Perc(x * y)),
            LengthOperation::Div => Ok(Length::Perc(x / y)),
        },
        (Length::Pixels(x), Length::Perc(y)) => match op {
            LengthOperation::Add => Err(LengthOperationError::PixelPercAddError),
            LengthOperation::Sub => Err(LengthOperationError::PixelPercSubError),
            LengthOperation::Mul => Ok(Length::Pixels((x as f32 * y) as i32)),
            LengthOperation::Div => Ok(Length::Pixels((x as f32 * y) as i32)),
        },
        (Length::Perc(x), Length::Pixels(y)) => match op {
            LengthOperation::Add => Err(LengthOperationError::PercPixelAddError),
            LengthOperation::Sub => Err(LengthOperationError::PercPixelSubError),
            LengthOperation::Mul => Ok(Length::Pixels((x * y as f32) as i32)),
            LengthOperation::Div => Ok(Length::Pixels((x / y as f32) as i32)),
        },
    }
}

// guh i hate myself
// i really need something to just distract my mind
// so i thought this crate would be a fun project
// but it's already a mess
// and i hate how it sounds so dry what im saying
// like i cant even say whats on my mind really
// i just hope someone will find this thing useful and it wont cause him many troubles
