use appcui::prelude::*;

#[allow(dead_code)]
pub struct SpotifySong {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: String,
}

impl SpotifySong {
    pub fn new(
        title: impl Into<String>,
        artist: impl Into<String>,
        album: impl Into<String>,
        duration: impl Into<String>,
    ) -> SpotifySong {
        Self {
            title: title.into(),
            artist: artist.into(),
            album: album.into(),
            duration: duration.into(),
        }
    }
}

impl components::listitem::ListItem for SpotifySong {
    fn columns_count() -> u16 {
        1
    }

    fn column(_index: u16) -> components::Column {
        components::Column::new("Songs", 255, TextAlignment::Left)
    }

    fn render_method(&self, _column_index: u16) -> Option<components::listitem::RenderMethod<'_>> {
        Some(components::listitem::RenderMethod::Custom)
    }

    fn paint(&self, _column_index: u32, width: u16, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        let char_attr = attr.unwrap_or(theme.text.normal);

        // Left side: icon + song title + artist
        let left = format!("♪ {}  •  {}", self.title, self.artist);

        // Right side: album + duration pinned to the far right edge
        let right = format!("{}  │  {}", self.album, self.duration);
        let right_len = right.chars().count() as i32;
        let right_x = (width as i32).saturating_sub(right_len + 1);

        // Wrap or clip left text so it never overlaps the right details
        let left_max_width = if right_x > 2 {
            (right_x - 2).max(1) as u16
        } else {
            width.saturating_sub(1)
        };

        let left_format = TextFormatBuilder::new()
            .position(0, 0)
            .attribute(char_attr)
            .align(TextAlignment::Left)
            .wrap_type(WrapType::SingleLineWrap(left_max_width))
            .build();
        surface.write_text(&left, &left_format);

        // Paint right side details if there is enough room
        if right_x > 0 && right_x < width as i32 {
            let right_format = TextFormatBuilder::new()
                .position(right_x, 0)
                .attribute(char_attr)
                .align(TextAlignment::Left)
                .chars_count(right.chars().count() as u16)
                .build();
            surface.write_text(&right, &right_format);
        }
    }

    fn matches(&self, text: &str) -> bool {
        let text_lower = text.to_lowercase();
        self.title.to_lowercase().contains(&text_lower)
            || self.artist.to_lowercase().contains(&text_lower)
            || self.album.to_lowercase().contains(&text_lower)
    }
}