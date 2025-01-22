use skia_safe::{Paint, Point, Rect, TextBlob, TileMode, Shader, Color4f};
use crate::core::{
    Overlay, OverlayError,
    helper::to_color_4f
};

// TODO: CREATE FILLED AND GRADIENT ELLIPSE DRAW FUNCS

impl Overlay {
    // TEXT FUNCTIONS -------------------------

    pub fn draw_text(
        &mut self,
        (x, y): (f32, f32),
        text: impl ToString,
        color: (u8, u8, u8, u8)
    ) -> Result <(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let text = text.to_string();

        // Convert the RGBA color tuple to Skia Color
        let color = to_color_4f(color);

        // Create a Paint object with the desired stroke width and color
        let mut paint = Paint::new(color, None);
        paint.set_anti_alias(true);

        let text_blob = TextBlob::new(text, &self.font).expect("Failed to create TextBlob");

        canvas.draw_text_blob(&text_blob, Point::new(x,y), &paint);

        Ok(())
    }


    pub fn draw_outlined_text(
        &mut self,
        (x, y): (f32, f32),
        text: &str,
        color: (u8, u8, u8, u8)
    ) -> Result<(), OverlayError> {
        // Outline
        self.draw_text((x - 1.0, y), text, (0, 0, 0, 255))?;
        self.draw_text((x + 1.0, y), text, (0, 0, 0, 255))?;
        self.draw_text((x - 1.0, y), text, (0, 0, 0, 255))?;
        self.draw_text((x + 1.0, y), text, (0, 0, 0, 255))?;
        // Main text
        self.draw_text((x, y), text, color)?;

        Ok(())
    }

    // LINE FUNCTIONS -------------------------

    pub fn draw_line(
        &mut self,
        start: (f32, f32),
        end: (f32, f32),
        stroke_width: f32,
        color: (u8, u8, u8, u8)
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color = to_color_4f(color);
        let mut paint = Paint::new(color, None);

        paint.set_stroke_width(stroke_width);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Stroke);

        canvas.draw_line(
            Point::new(start.0, start.1),
            Point::new(end.0, end.1),
            &paint
        );
        Ok(())
    }

    pub fn draw_gradient_line(
        &mut self,
        start: (f32, f32),
        end: (f32, f32),
        stroke_width: f32,
        color1: (u8, u8, u8, u8),
        color2: (u8, u8, u8, u8),
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color1 = to_color_4f(color1);
        let color2 = to_color_4f(color2);

        let colors: &[Color4f] = &[color1, color2];

        let points = [
            Point::new(start.0, start.1),
            Point::new(end.0, end.1)
        ];

        let mut paint = Paint::default();
        paint.set_stroke_width(stroke_width);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Stroke);

        let shader = Shader::linear_gradient(
            (Point::new(start.0, start.1), Point::new(end.0, end.1)),
            colors,
            None,
            TileMode::Clamp,
            None,
            None,
        );

        paint.set_shader(shader);

        canvas.draw_line(points[0], points[1], &paint);

        Ok(())
    }

    // RECTANGLE FUNCTIONS --------------------

    pub fn draw_rect(
        &mut self,
        (x, y): (f32, f32),
        (width, height): (f32, f32),
        stroke_width: f32,
        color: (u8, u8, u8, u8)
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        // Convert the RGBA color tuple to Skia Color
        let color = to_color_4f(color);

        // Create a Paint object with the desired stroke width and color
        let mut paint = Paint::new(color, None);
        paint.set_stroke_width(stroke_width); // Set the stroke width for the rectangle

        // Create a rectangle with the provided position and size
        let rect = Rect::new(x, y, x + width, y + height);

        // Draw the rectangle on the canvas
        canvas.draw_rect(rect, &paint);

        Ok(())
    }

    pub fn draw_filled_rect(
        &mut self,
        (x, y): (f32, f32),
        (width, height): (f32, f32),
        color: (u8, u8, u8, u8)
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color = to_color_4f(color);

        let mut paint = Paint::new(color, None);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Fill);

        let rect = Rect::new(x, y, x + width, y + height);
        canvas.draw_rect(rect, &paint);

        Ok(())
    }

    pub fn draw_gradient_rect(
        &mut self,
        (x, y): (f32, f32),
        (width, height): (f32, f32),
        color1: (u8, u8, u8, u8),
        color2: (u8, u8, u8, u8),
        is_vertical: bool,
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color1 = to_color_4f(color1);
        let color2 = to_color_4f(color2);

        let colors: &[Color4f] = &[color1, color2];

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Fill);

        let (start, end) = if is_vertical {
            (Point::new(x, y), Point::new(x, y + height))
        } else {
            (Point::new(x, y), Point::new(x + width, y))
        };

        let shader = Shader::linear_gradient(
            (start, end),
            colors,
            None,
            TileMode::Clamp,
            None,
            None,
        );

        paint.set_shader(shader);
        canvas.draw_rect(Rect::new(x, y, x + width, y + height), &paint);

        Ok(())
    }


    // ROUNDED RECTANGLE FUNCTIONS ------------

    pub fn draw_rounded_rect(
        &mut self,
        (x, y): (f32, f32),
        (width, height): (f32, f32),
        radius: f32,
        stroke_width: f32,
        color: (u8, u8, u8, u8)
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color = to_color_4f(color);

        let mut paint = Paint::new(color, None);
        paint.set_stroke_width(stroke_width);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Stroke);

        let rect = Rect::new(x, y, x + width, y + height);
        canvas.draw_round_rect(rect, radius, radius, &paint);

        Ok(())
    }

    pub fn draw_filled_rounded_rect(
        &mut self,
        (x, y): (f32, f32),
        (width, height): (f32, f32),
        radius: f32,
        color: (u8, u8, u8, u8)
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color = to_color_4f(color);

        let mut paint = Paint::new(color, None);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Fill);

        let rect = Rect::new(x, y, x + width, y + height);
        canvas.draw_round_rect(rect, radius, radius, &paint);

        Ok(())
    }

    pub fn draw_gradient_rounded_rect(
        &mut self,
        (x, y): (f32, f32),
        (width, height): (f32, f32),
        radius: f32,
        color1: (u8, u8, u8, u8),
        color2: (u8, u8, u8, u8),
        is_vertical: bool,
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color1 = to_color_4f(color1);
        let color2 = to_color_4f(color2);

        let colors: &[Color4f] = &[color1, color2];

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Fill);

        let (start, end) = if is_vertical {
            (Point::new(x, y), Point::new(x, y + height))
        } else {
            (Point::new(x, y), Point::new(x + width, y))
        };

        let shader = Shader::linear_gradient(
            (start, end),
            colors,
            None,
            TileMode::Clamp,
            None,
            None,
        );

        paint.set_shader(shader);
        let rect = Rect::new(x, y, x + width, y + height);
        canvas.draw_round_rect(rect, radius, radius, &paint);

        Ok(())
    }

    // CIRCLE FUNCTIONS ----------------------

    pub fn draw_circle(
        &mut self,
        center: (f32, f32),
        radius: f32,
        stroke_width: f32,
        color: (u8, u8, u8, u8)
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color = to_color_4f(color);

        let mut paint = Paint::new(color, None);
        paint.set_stroke_width(stroke_width);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Stroke);

        canvas.draw_circle(Point::new(center.0, center.1), radius, &paint);

        Ok(())
    }

    pub fn draw_filled_circle(
        &mut self,
        center: (f32, f32),
        radius: f32,
        color: (u8, u8, u8, u8)
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color = to_color_4f(color);

        let mut paint = Paint::new(color, None);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Fill);

        canvas.draw_circle(Point::new(center.0, center.1), radius, &paint);

        Ok(())
    }

    pub fn draw_gradient_circle(
        &mut self,
        center: (f32, f32),
        radius: f32,
        color1: (u8, u8, u8, u8),
        color2: (u8, u8, u8, u8),
        is_radial: bool,
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color1 = to_color_4f(color1);
        let color2 = to_color_4f(color2);

        let colors: &[Color4f] = &[color1, color2];

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Fill);

        let shader = if is_radial {
            Shader::radial_gradient(
                Point::new(center.0, center.1),
                radius,
                colors,
                None,
                TileMode::Clamp,
                None,
                None,
            )
        } else {
            Shader::linear_gradient(
                (
                    Point::new(center.0 - radius, center.1),
                    Point::new(center.0 + radius, center.1)
                ),
                colors,
                None,
                TileMode::Clamp,
                None,
                None,
            )
        };

        paint.set_shader(shader);
        canvas.draw_circle(Point::new(center.0, center.1), radius, &paint);

        Ok(())
    }

    // ELLIPSE FUNCTIONS ---------------------

    pub fn draw_ellipse(
        &mut self,
        center: (f32, f32),
        (radius_x, radius_y): (f32, f32),
        stroke_width: f32,
        color: (u8, u8, u8, u8)
    ) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();

        let color = to_color_4f(color);

        let mut paint = Paint::new(color, None);
        paint.set_stroke_width(stroke_width);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Stroke);

        let rect = Rect::new(
            center.0 - radius_x,
            center.1 - radius_y,
            center.0 + radius_x,
            center.1 + radius_y
        );
        canvas.draw_oval(rect, &paint);

        Ok(())
    }
}