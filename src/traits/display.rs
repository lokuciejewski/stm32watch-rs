use embassy_time::Duration;

#[allow(async_fn_in_trait)]
pub trait Display {
    const SCREEN_WIDTH_CHARS: usize;
    const SCREEN_HEIGHT_CHARS: usize;

    const CHAR_WIDTH_PIXELS: usize;
    const CHAR_HEIGHT_PIXELS: usize;

    const SCREEN_WIDTH_PIXELS: usize;
    const SCREEN_HEIGHT_PIXELS: usize;


    type Coordinates;

    fn move_cursor(&mut self, new_position: Self::Coordinates);
    fn home_cursor(&mut self);

    fn print_string(&mut self, string: &str);
    fn print_at(&mut self, string: &str, position: Self::Coordinates);
    fn print_at_grid(&mut self, string: &str, position: Self::Coordinates, snap_to_grid: bool);

    async fn scroll_text(&mut self, string: &str, delay_ms: Duration);

    fn clear(&mut self);
}
