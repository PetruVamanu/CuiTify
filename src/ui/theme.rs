use appcui::prelude::*;

macro_rules! set_states {
    ($target:expr, $normal:literal, $focused:literal, $hovered:literal, $inactive:literal, $pressed:literal) => {
        $target.normal = charattr!($normal);
        $target.focused = charattr!($focused);
        $target.hovered = charattr!($hovered);
        $target.inactive = charattr!($inactive);
        $target.pressed_or_selected = charattr!($pressed);
    };
}

/// Creates a hacker/terminal-inspired theme with deep blacks, phosphor greens, and neon accents.
pub fn hacker_theme() -> Theme {
    let mut theme = Theme::new(Themes::Default);

    // Desktop background (pitch black)
    theme.desktop.character = char!("' ',black,black");

    // Window interior and title bar
    theme.window.normal = charattr!("green,black");
    theme.window.inactive = charattr!("darkgreen,black");
    theme.window.bar.focus = charattr!("black,green");
    theme.window.bar.normal = charattr!("darkgreen,black");
    theme.window.bar.resizing = charattr!("black,aqua");
    theme.window.bar.close_button = charattr!("red,green");
    theme.window.bar.maximize_button = charattr!("black,green");
    theme.window.bar.hotkey = charattr!("darkgreen,green");

    // Control borders and splitter lines
    set_states!(theme.border, "darkgreen,black", "green,black", "aqua,black", "gray,black", "black,green");
    set_states!(theme.lines, "darkgreen,black", "green,black", "aqua,black", "gray,black", "black,green");

    // Typography
    theme.text.normal = charattr!("green,black");
    theme.text.focused = charattr!("white,black");
    theme.text.inactive = charattr!("darkgreen,black");
    theme.text.hot_key = charattr!("aqua,black");
    theme.text.hovered = charattr!("white,black");
    theme.text.highlighted = charattr!("aqua,black");
    theme.text.enphasized_1 = charattr!("aqua,black");
    theme.text.enphasized_2 = charattr!("green,black");
    theme.text.enphasized_3 = charattr!("yellow,black");
    theme.text.error = charattr!("red,black");
    theme.text.warning = charattr!("yellow,black");

    // Symbols (arrows, checks, etc.)
    theme.symbol.checked = charattr!("green,black");
    theme.symbol.unchecked = charattr!("darkgreen,black");
    theme.symbol.arrows = charattr!("green,black");
    theme.symbol.inactive = charattr!("darkgreen,black");
    theme.symbol.hovered = charattr!("black,green");

    // List item selection (classic hacker reverse-video cursor highlight)
    theme.list_current_item.focus = charattr!("black,green");
    theme.list_current_item.normal = charattr!("green,black");
    theme.list_current_item.over_selection = charattr!("black,darkgreen");
    theme.list_current_item.over_inactive = charattr!("darkgreen,black");
    theme.list_current_item.selected = charattr!("black,green");
    theme.list_current_item.icon = charattr!("green,black");

    // Column headers
    set_states!(theme.header.text, "green,black", "white,darkgreen", "black,green", "darkgreen,black", "black,green");

    // Search bar
    theme.searchbar.normal = charattr!("green,black");
    theme.searchbar.focused = charattr!("white,darkgreen");
    theme.searchbar.count = charattr!("aqua,black");

    // Scrollbars
    set_states!(theme.scrollbar.arrow, "darkgreen,black", "green,black", "white,darkgreen", "black,black", "black,green");
    set_states!(theme.scrollbar.bar, "darkgreen,black", "darkgreen,black", "darkgreen,black", "black,black", "green,black");
    set_states!(theme.scrollbar.position, "green,black", "black,green", "white,green", "black,black", "black,green");

    // Buttons
    set_states!(theme.button.regular.text, "green,black", "black,green", "black,green", "darkgreen,black", "black,darkgreen");
    set_states!(theme.button.regular.hotkey, "aqua,black", "black,green", "black,green", "darkgreen,black", "black,darkgreen");
    theme.button.regular.shadow = charattr!("black,black");

    theme
}
