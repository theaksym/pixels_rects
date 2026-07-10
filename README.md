# pixels_rects
A Rust crate for creating isolated rectangular areas to render on with a pixel buffer.

It provides structs, enums and methods to ease out your work with pixel buffers, such as those in `pixels` or `minifb`.

# Example
Let's draw an image, because that's what this crate is for! For me it's an image of my cute cat.
First, we define the required variables:
```rust
// load your image however you choose

let rect = Rect::from_pos_size(
    Length::Perc(0.5),
    Length::Perc(0.5),
    Length::Pixels(200),
    Length::Pixels(200),
)
.unwrap();

let draw_data = RectDrawData::new(cat_photo_colors, image_width, image_height);

let size_data = DrawSizeData::new(
    800,
    600,
    800 as f32,
    600 as f32,
);
```
Here we create a `Rect` in the middle of it's surrounding space and a size of 200 pixels in each axis. The `draw_data` consists of the color buffer and image size. In `size_data` we provide data about space surrounding the `Rect`. First two values describe the size of the buffer we render on. The last two floats describe values that will be used in `Length::Perc` related operations - here we pass in the buffer size again, because we want the `Rect` to be in the middle of the screen.
With this data, we can finally draw the image!
```rust 
draw_rect(&rect, &draw_data, &size_data, DrawScaleMode::Fill, &mut buffer);
```
We pass references to the data, mutable reference to the pixel buffer, and a `DrawScaleMode` variant which tells the function how to scale our image (`Fill` means that the image will be scaled in each axis independently to fill the whole `Rect`). Now you should see the desired image!
