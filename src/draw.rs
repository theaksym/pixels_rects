use crate::{
    length::{Length, to_i32},
    rect::Rect,
};
use num_traits::PrimInt;
use std::f32;

/// Data to be drawn in a Rect
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RectDrawData {
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
}
impl RectDrawData {
    pub fn new(buffer: &[u8], width: u32, height: u32) -> Self {
        Self {
            buffer: buffer.to_vec(),
            width,
            height,
        }
    }
}

/// Size data used for drawing in a Rect
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DrawSizeData {
    /// Width of the buffer
    pub buffer_width: u32,

    /// Height of the buffer
    pub buffer_height: u32,

    /// Used for Lengths such as Length::Perc() etc.
    pub ref_width: f32,

    /// Used for Lengths such as Length::Perc() etc.
    pub ref_height: f32,
}
impl DrawSizeData {
    pub fn new(buffer_width: u32, buffer_height: u32, ref_width: f32, ref_height: f32) -> Self {
        Self {
            buffer_width,
            buffer_height,
            ref_width,
            ref_height,
        }
    }
}

pub enum DrawScaleMode {
    /// Won't resize the buffer to fit the Rect
    None,

    /// Resizes the buffer in each axis independently to fill the Rect
    /// May cause it to warp uncontrollably
    Fill,

    /// Resizes the buffer in all axes by the same value, until it reaches the edges
    /// Won't modify the image, but can make it extend outside the Rect
    Fit,
}

/// Draws the data into a Rect, scaling it appropriately.
pub fn draw_rect(
    rect: &Rect,
    draw_data: &RectDrawData,
    size_data: &DrawSizeData,
    scale_mode: DrawScaleMode,
    buffer: &mut [u8],
) {
    // MOTHER OF ALL VIRTUEEEES

    match scale_mode {
        DrawScaleMode::None => draw_rect_none(rect, draw_data, size_data, buffer),
        DrawScaleMode::Fill => draw_rect_fill(rect, draw_data, size_data, buffer),
        DrawScaleMode::Fit => draw_rect_fit(rect, draw_data, size_data, buffer),
    }
}
fn draw_rect_none(
    rect: &Rect,
    draw_data: &RectDrawData,
    size_data: &DrawSizeData,
    buffer: &mut [u8],
) {
    let (pos_x, pos_y) = {
        let (temp_x, temp_y) = rect.get_pos_pixels(size_data.ref_width, size_data.ref_height);

        to_i32(temp_x, temp_y, size_data.ref_width, size_data.ref_height)
    };

    let ref_x = pos_x - draw_data.width as i32 / 2;
    let ref_y = pos_y - draw_data.height as i32 / 2;

    for x in 0..draw_data.width {
        for y in 0..draw_data.height {
            let new_x = ref_x + x as i32;
            let new_y = ref_y + y as i32;

            if new_x >= 0
                && new_x < size_data.buffer_width as i32
                && new_y >= 0
                && new_y < size_data.buffer_height as i32
                && rect.contains_point(new_x, new_y, size_data.ref_width, size_data.ref_height)
            {
                let draw_idx = idx(x, y, draw_data.width) as usize * 4;
                let buffer_idx = idx(new_x, new_y, size_data.buffer_width as i32) as usize * 4;

                for i in 0..4 {
                    buffer[buffer_idx + i] = draw_data.buffer[draw_idx + i];
                }
            }
        }
    }
}
fn draw_rect_fill(
    rect: &Rect,
    draw_data: &RectDrawData,
    size_data: &DrawSizeData,
    buffer: &mut [u8],
) {
    let (pos_x, pos_y) = {
        let (temp_x, temp_y) = rect.get_pos_pixels(size_data.ref_width, size_data.ref_height);

        to_i32(temp_x, temp_y, size_data.ref_width, size_data.ref_height)
    };

    let (width, height) = {
        let (temp_x, temp_y) = rect.get_size_pixels(size_data.ref_width, size_data.ref_height);

        to_i32(temp_x, temp_y, size_data.ref_width, size_data.ref_height)
    };

    let ref_x = pos_x - width as i32 / 2;
    let ref_y = pos_y - height as i32 / 2;

    for y in 0..height {
        for x in 0..width {
            let new_x = ref_x + x as i32;
            let new_y = ref_y + y as i32;

            if new_x >= 0
                && new_x < size_data.buffer_width as i32
                && new_y >= 0
                && new_y < size_data.buffer_height as i32
            {
                let perc_x = x as f32 / width as f32;
                let perc_y = y as f32 / height as f32;

                let colors = sample(&draw_data, perc_x, perc_y);

                let idx = idx(new_x, new_y, size_data.buffer_width as i32) as usize * 4;

                for i in 0..4 {
                    buffer[idx + i] = colors[i];
                }
            }
        }
    }
}
fn draw_rect_fit(
    rect: &Rect,
    draw_data: &RectDrawData,
    size_data: &DrawSizeData,
    buffer: &mut [u8],
) {
    let (pos_x, pos_y) = {
        let (temp_x, temp_y) = rect.get_pos_pixels(size_data.ref_width, size_data.ref_height);

        to_i32(temp_x, temp_y, size_data.ref_width, size_data.ref_height)
    };

    let (width, height) = {
        let (temp_x, temp_y) = rect.get_size_pixels(size_data.ref_width, size_data.ref_height);

        to_i32(temp_x, temp_y, size_data.ref_width, size_data.ref_height)
    };

    let scale_perc = if draw_data.width < draw_data.height {
        draw_data.width as f32 / width as f32
    } else {
        draw_data.height as f32 / height as f32
    };

    let (ref_width, ref_height) = (
        Length::Pixels((draw_data.width as f32 / scale_perc) as i32),
        Length::Pixels((draw_data.height as f32 / scale_perc) as i32),
    );

    let (ref_width_pixels, ref_height_pixels) = to_i32(
        ref_width,
        ref_height,
        size_data.ref_width,
        size_data.ref_height,
    );

    let ref_x = pos_x - ref_width_pixels as i32 / 2;
    let ref_y = pos_y - ref_height_pixels as i32 / 2;

    for x in 0..ref_width_pixels {
        for y in 0..ref_height_pixels {
            let new_x = ref_x + x as i32;
            let new_y = ref_y + y as i32;

            if new_x >= 0
                && new_x < size_data.buffer_width as i32
                && new_y >= 0
                && new_y < size_data.buffer_height as i32
                && rect.contains_point(new_x, new_y, size_data.ref_width, size_data.ref_height)
            {
                let perc_x = x as f32 / ref_width_pixels as f32;
                let perc_y = y as f32 / ref_height_pixels as f32;

                let colors = sample(&draw_data, perc_x, perc_y);

                let idx = idx(new_x, new_y, size_data.buffer_width as i32) as usize * 4;

                for i in 0..4 {
                    buffer[idx + i] = colors[i];
                }
            }
        }
    }
}

/// Samples colors from some point expressed in percentages.
pub fn sample(data: &RectDrawData, x: f32, y: f32) -> [u8; 4] {
    let new_x = (data.width as f32 * x) as u32;
    let new_y = (data.height as f32 * y) as u32;

    let idx = idx(new_x, new_y, data.width) as usize * 4;

    let mut temp = [0; 4];

    for i in 0..4 {
        temp[i] = data.buffer[idx + i];
    }

    temp
}

/// Convert an XY position to a 1D index.
pub fn idx<T: PrimInt>(x: T, y: T, width: T) -> T {
    y * width + x
}

/// Convert a 1D index to an XY position.
/// kinda sick of this formal tone bro
pub fn xy<T: PrimInt>(idx: T, width: T) -> (T, T) {
    let x = idx % width;
    let y = idx / width;

    (x, y)
}

/// Converts a u32 buffer to a u8 one
pub fn buffer_u32_to_u8(buffer: &[u32]) -> Vec<u8> {
    let mut new_buffer = Vec::with_capacity(buffer.len() * 4);

    for color in buffer {
        let color_u8 = color.to_le_bytes();
        new_buffer.extend(color_u8.iter());
    }

    new_buffer
}
