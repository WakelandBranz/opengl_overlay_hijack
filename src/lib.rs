mod core;

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use crate::core::Overlay;
    extern crate fps_counter;

    use fps_counter::*;

    #[test]
    fn test_overlay() {
        let mut overlay = Overlay::new("Tahoma", 18.0);

        // Initialize overlay
        match overlay.init() {
            Ok(_) => println!("Successfully initialized overlay"),
            Err(_) => println!("Failed to initialize overlay")
        };

        // Startup overlay rendering
        match overlay.startup_renderer(false) {
            Ok(_) => println!("Successfully started renderer"),
            Err(_) => println!("Failed to startup renderer"),
        };

        println!("Successfully initialized, rendering for 15 seconds now..\n");

        let red = (255, 51, 0, 255);
        let green = (0, 255, 51, 255);
        let blue = (0, 51, 255, 255);
        let yellow = (255, 255, 0, 255);
        let purple = (255, 0, 255, 255);
        let cyan = (0, 255, 255, 255);

        let mut fps_counter = FPSCounter::default();

        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(15) {
            overlay.begin_scene().unwrap();
            // Text at the top
            overlay.draw_text(
                (10.0, 30.0),
                "https://github.com/WakelandBranz/nvidia-overlay-hijack",
                (255, 255, 255, 255),
            ).expect("Failed to draw text");

            overlay.draw_text(
                (10.0, 50.0),
                format!("Shape Showcase -> FPS: {}", fps_counter.tick()),
                (255, 255, 255, 255),
            ).expect("Failed to draw text");

            // Basic shapes
            overlay.draw_rect(
                (10.0, 100.0),
                (100.0, 80.0),
                2.0,
                yellow
            ).expect("Failed to draw rectangle");

            overlay.draw_filled_rect(
                (120.0, 100.0),
                (100.0, 80.0),
                green
            ).expect("Failed to draw filled rectangle");

            overlay.draw_gradient_rect(
                (230.0, 100.0),
                (100.0, 80.0),
                red,
                blue,
                true
            ).expect("Failed to draw gradient rectangle");

            // Rounded rectangles
            overlay.draw_rounded_rect(
                (10.0, 200.0),
                (100.0, 80.0),
                10.0,
                2.0,
                purple
            ).expect("Failed to draw rounded rectangle");

            overlay.draw_filled_rounded_rect(
                (120.0, 200.0),
                (100.0, 80.0),
                10.0,
                cyan
            ).expect("Failed to draw filled rounded rectangle");

            overlay.draw_gradient_rounded_rect(
                (230.0, 200.0),
                (100.0, 80.0),
                10.0,
                green,
                purple,
                false
            ).expect("Failed to draw gradient rounded rectangle");

            // Circles and Ellipses
            overlay.draw_circle(
                (60.0, 350.0),
                30.0,
                2.0,
                yellow
            ).expect("Failed to draw circle");

            overlay.draw_filled_circle(
                (170.0, 350.0),
                30.0,
                blue
            ).expect("Failed to draw filled circle");

            overlay.draw_gradient_circle(
                (280.0, 350.0),
                30.0,
                red,
                blue,
                true
            ).expect("Failed to draw gradient circle (radial)");

            // Ellipses
            overlay.draw_ellipse(
                (60.0, 450.0),
                (40.0, 25.0),
                2.0,
                green
            ).expect("Failed to draw ellipse");

            // TODO: CREATE FILLED AND GRADIENT ELLIPSE DRAW FUNCS

            // Regular line
            overlay.draw_line(
                (400.0, 100.0),
                (500.0, 150.0),
                2.0,
                yellow
            ).expect("Failed to draw line");

            // Gradient line
            overlay.draw_gradient_line(
                (400.0, 200.0),
                (500.0, 250.0),
                3.0,
                red,
                blue
            ).expect("Failed to draw gradient line");

            overlay.present_scene().unwrap();
        }
    }
}
