use frogquad::{
    Color,
    conf::Conf,
    input::mouse_position,
    shapes::{draw_line, draw_rectangle_lines},
    text::{draw_multiline_text, measure_multiline_text, wrap_text},
};

static LOREM: &str = "Lorem ipsum odor amet, consectetuer adipiscing elit. Ultrices nostra volutpat facilisis magna mus. Rhoncus tempor feugiat netus maecenas pretium leo vitae. Eros aliquet maecenas eu diam aliquet varius hac elementum. Sociosqu platea per ultricies vitae praesent mauris nostra ridiculus. Est cursus pulvinar efficitur mus vel leo. Integer et nec eleifend non leo. Lorem rutrum ultrices potenti facilisis hendrerit facilisi metus sit. AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA

Intentional newlines
are preserved.";

struct TextWrapApp {}

impl frogquad::App for TextWrapApp {
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        let font_size = 24;

        fc.clear_background(Color::BLACK);

        let maximum_line_length = f32::max(20.0, mouse_position(fc).0 - 20.0);
        let text = wrap_text(
            LOREM,
            fc.default_font(),
            font_size,
            1.0,
            maximum_line_length,
        );
        let dimensions =
            measure_multiline_text(&text, fc.default_font(), font_size, 1.0, Some(1.0));

        draw_multiline_text(
            fc,
            &text,
            20.0,
            20.0 + dimensions.offset_y,
            font_size as f32,
            Some(1.0),
            Color::WHITE,
        );
        draw_rectangle_lines(
            fc,
            20.0,
            20.0,
            dimensions.width,
            dimensions.height,
            2.0,
            Color::BLUE,
        );
        draw_line(
            fc,
            20.0 + maximum_line_length,
            0.0,
            20.0 + maximum_line_length,
            fc.screen_height(),
            1.0,
            Color::RED,
        );
    }
}

fn main() {
    frogquad::start(Conf::from_title("Text Wrap"), |_fc| {
        Box::new(TextWrapApp {})
    });
}
