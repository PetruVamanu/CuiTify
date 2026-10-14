use appcui::prelude::*;

#[derive(Clone, Copy)]
pub enum PlaylistTag {
    Public,
    Private,
    Collaborative,
}

impl PlaylistTag {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlaylistTag::Public => "Public",
            PlaylistTag::Private => "Private",
            PlaylistTag::Collaborative => "Collab",
        }
    }
}

pub struct SpotifyPlaylist {
    pub name: String,
    pub track_count: u32,
    pub owner: String,
    pub tag: PlaylistTag,
}

impl SpotifyPlaylist {
    pub fn new(name: impl Into<String>, track_count: u32, owner: impl Into<String>, tag: PlaylistTag) -> Self {
        Self {
            name: name.into(),
            track_count,
            owner: owner.into(),
            tag,
        }
    }
}

impl components::listitem::ListItem for SpotifyPlaylist {
    fn columns_count() -> u16 {
        1
    }

    fn column(_index: u16) -> components::Column {
        components::Column::new("Playlists", 255, TextAlignment::Left)
    }

    fn render_method(&self, _column_index: u16) -> Option<components::listitem::RenderMethod<'_>> {
        Some(components::listitem::RenderMethod::Custom)
    }

    fn paint(&self, _column_index: u32, width: u16, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        let char_attr = attr.unwrap_or(theme.text.normal);

        let left = format!("♪ {}", self.name);
        let left_format = TextFormatBuilder::new()
            .position(0, 0)
            .attribute(char_attr)
            .align(TextAlignment::Left)
            .chars_count(left.chars().count() as u16)
            .build();
        surface.write_text(&left, &left_format);

        let right = format!("{} tracks  •  {} [{}]", self.track_count, self.owner, self.tag.as_str());
        let right_len = right.chars().count() as i32;
        let right_x = (width as i32).saturating_sub(right_len + 1).max(left.chars().count() as i32 + 2);

        let right_format = TextFormatBuilder::new()
            .position(right_x, 0)
            .attribute(char_attr)
            .align(TextAlignment::Left)
            .chars_count(right.chars().count() as u16)
            .build();
        surface.write_text(&right, &right_format);
    }

    fn matches(&self, text: &str) -> bool {
        let text_lower = text.to_lowercase();
        self.name.to_lowercase().contains(&text_lower)
            || self.owner.to_lowercase().contains(&text_lower)
    }
}