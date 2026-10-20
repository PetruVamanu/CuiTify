use appcui::prelude::*;

#[CustomControl(overwrite: OnPaint)]
pub struct PlayerBar {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub current_time_sec: u32,
    pub total_time_sec: u32,
    pub is_playing: bool,
    pub shuffle: bool,
    pub repeat: bool,
    pub volume: u8, // 0 - 100
}

impl PlayerBar {
    pub fn new() -> Self {
        Self {
            base: ControlBase::new(layout!("d:f"), true),
            title: "Get Lucky (feat. Pharrell Williams and Nile Rodgers)".to_string(),
            artist: "Daft Punk, Pharrell Williams, Nile Rodgers".to_string(),
            album: "Random Access Memories".to_string(),
            current_time_sec: 89,   // 1:29
            total_time_sec: 369,    // 6:09
            is_playing: true,
            shuffle: false,
            repeat: false,
            volume: 49,
        }
    }

    fn format_time(seconds: u32) -> String {
        let mins = seconds / 60;
        let secs = seconds % 60;
        format!("{}:{:02}", mins, secs)
    }
}

impl OnPaint for PlayerBar {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let sz = self.size();
        let width = sz.width as i32;
        let height = sz.height as i32;

        if width < 10 || height < 1 {
            return;
        }

        // Color attributes
        let white_bold = CharAttribute::new(Color::White, Color::Black, CharFlags::Bold);
        let normal_white = CharAttribute::new(Color::White, Color::Black, CharFlags::None);
        let dim_gray = CharAttribute::new(Color::Silver, Color::Black, CharFlags::None);
        let dark_gray = CharAttribute::new(Color::Gray, Color::Black, CharFlags::None);
        let green_color = theme.text.normal;

        // Clear background with black
        surface.clear(Character::with_attributes(' ', theme.window.normal));

        // -------------------------------------------------------------
        // Row 0: Song Title (left) + Controls (right)
        // -------------------------------------------------------------
        let controls = format!(
            "|◀   {}   ▶|   {} Shuffle   {} Repeat",
            if self.is_playing { "|| Pause" } else { " ▶ Play " },
            if self.shuffle { "⇄" } else { "⇄" },
            if self.repeat { "↻" } else { "↻" }
        );
        let controls_len = controls.chars().count() as i32;
        let controls_x = (width - controls_len - 1).max(0);

        // Draw song title
        let title_max_width = if controls_x > 2 {
            (controls_x - 2) as u16
        } else {
            width.saturating_sub(1) as u16
        };
        let title_format = TextFormatBuilder::new()
            .position(1, 0)
            .attribute(white_bold)
            .align(TextAlignment::Left)
            .wrap_type(WrapType::SingleLineWrap(title_max_width))
            .build();
        surface.write_text(&self.title, &title_format);

        // Draw controls on the right
        if controls_x > 0 && controls_x < width {
            let ctrl_format = TextFormatBuilder::new()
                .position(controls_x, 0)
                .attribute(normal_white)
                .align(TextAlignment::Left)
                .chars_count(controls_len as u16)
                .build();
            surface.write_text(&controls, &ctrl_format);
        }

        // -------------------------------------------------------------
        // Row 1: Artist · Album
        // -------------------------------------------------------------
        if height >= 2 {
            let subtitle = format!("{} · {}", self.artist, self.album);
            let sub_format = TextFormatBuilder::new()
                .position(1, 1)
                .attribute(dim_gray)
                .align(TextAlignment::Left)
                .wrap_type(WrapType::SingleLineWrap(width.saturating_sub(2) as u16))
                .build();
            surface.write_text(&subtitle, &sub_format);
        }

        // -------------------------------------------------------------
        // Row 2: Progress bar + Timestamps + Volume
        // -------------------------------------------------------------
        if height >= 3 {
            let current_str = Self::format_time(self.current_time_sec);
            let total_str = Self::format_time(self.total_time_sec);

            // Left timestamp: "1:29"
            surface.write_string(1, 2, &current_str, dim_gray, false);

            // Right side: Volume (e.g. "■■■■□□□□  49%")
            let total_bars = 8usize;
            let filled_bars = ((self.volume as usize * total_bars) + 50) / 100;
            let mut vol_str = String::with_capacity(16);
            for i in 0..total_bars {
                if i < filled_bars {
                    vol_str.push('■');
                } else {
                    vol_str.push('□');
                }
            }
            let vol_full_text = format!("{}  {:2}%", vol_str, self.volume);
            let vol_len = vol_full_text.chars().count() as i32;
            let vol_x = (width - vol_len - 1).max(0);

            if vol_x > 0 {
                surface.write_string(vol_x, 2, &vol_full_text, dim_gray, false);
            }

            // Duration right before volume: "6:09"
            let total_time_len = total_str.chars().count() as i32;
            let total_time_x = (vol_x - total_time_len - 3).max(0);
            if total_time_x > 0 {
                surface.write_string(total_time_x, 2, &total_str, dim_gray, false);
            }

            // Slider track between current_str and total_str
            let bar_start_x = (1 + current_str.chars().count() as i32 + 2).min(width);
            let bar_end_x = (total_time_x - 2).max(bar_start_x);

            let bar_width = bar_end_x - bar_start_x;
            if bar_width > 2 {
                let ratio = if self.total_time_sec > 0 {
                    (self.current_time_sec as f32 / self.total_time_sec as f32).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let playhead_rel = (ratio * (bar_width - 1) as f32).round() as i32;
                let playhead_x = bar_start_x + playhead_rel;

                // Elapsed line (green single horizontal line)
                if playhead_x > bar_start_x {
                    surface.fill_horizontal_line(
                        bar_start_x,
                        2,
                        playhead_x - 1,
                        Character::with_attributes('─', green_color),
                    );
                }

                // Playhead dot (green circle)
                surface.write_char(playhead_x, 2, Character::with_attributes('●', green_color));

                // Remaining line (dark gray horizontal line)
                if playhead_x < bar_end_x {
                    surface.fill_horizontal_line(
                        playhead_x + 1,
                        2,
                        bar_end_x,
                        Character::with_attributes('─', dark_gray),
                    );
                }
            }
        }
    }
}
