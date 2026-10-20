mod ui;

use appcui::prelude::*;
use ui::components::player_bar::PlayerBar;
use ui::components::spotify_playlist::{PlaylistTag, SpotifyPlaylist};
use ui::components::spotify_song::SpotifySong;

fn one_window_app() -> Window {
    let mut win = window!("CuiTify,d:f");
    let mut vs = vsplitter!("l:0,r:0,t:0,b:6,pos:30%,flags:MergeBorders");

    let mut hs = hsplitter!("x:0,y:0,w:100%,h:100%,pos:30%");
    let mut bottom_panel = Panel::new("", layout!("l:0,r:0,b:0,h:6"));
    bottom_panel.add(PlayerBar::new());

    let mut playlist_pane = panel!("Playlists,l:0,r:0,t:0,b:0");
    let mut library_pane = panel!("Library,l:0,r:0,t:0,b:0");

    let mut playlist_list = listview!("SpotifyPlaylist,l:0,r:0,t:0,b:0,vm:Columns(1),flags:SearchBar|MergeBorders");
    let mut library_list = listview!("SpotifyPlaylist,l:0,r:0,t:0,b:0,vm:Columns(1),flags:SearchBar|MergeBorders");

    playlist_list.add(SpotifyPlaylist::new("Daily Mix 1", 25, "Spotify", PlaylistTag::Public));
    playlist_list.add(SpotifyPlaylist::new("Chill Beats", 42, "Petri", PlaylistTag::Private));
    playlist_list.add(SpotifyPlaylist::new("Road Trip", 118, "Friends", PlaylistTag::Collaborative));
    playlist_list.add(SpotifyPlaylist::new("Favorites", 89, "Petri", PlaylistTag::Public));
    playlist_list.add(SpotifyPlaylist::new("Coding Focus", 64, "Spotify", PlaylistTag::Public));

    library_list.add(SpotifyPlaylist::new("Coding Focus", 64, "Spotify", PlaylistTag::Public));
    playlist_pane.add(playlist_list);
    library_pane.add(library_list);

    let mut right_pane = panel!("Songs,l:0,r:0,t:0,b:0");
    let mut song_list = listview!("SpotifySong,l:0,r:0,t:0,b:0,vm:Columns(1),flags:SearchBar|MergeBorders");
    song_list.add(SpotifySong::new("Starboy", "The Weeknd", "Starboy", "3:50"));
    song_list.add(SpotifySong::new("Blinding Lights", "The Weeknd", "After Hours", "3:20"));
    song_list.add(SpotifySong::new("Midnight City", "M83", "Hurry Up, We're Dreaming", "4:03"));
    song_list.add(SpotifySong::new("Instant Crush", "Daft Punk ft. Julian Casablancas", "Random Access Memories", "5:37"));
    song_list.add(SpotifySong::new("Get Lucky", "Daft Punk ft. Pharrell Williams", "Random Access Memories", "4:08"));
    right_pane.add(song_list);

    hs.add(hsplitter::Panel::Top, playlist_pane);
    hs.add(hsplitter::Panel::Bottom, library_pane);

    vs.add(vsplitter::Panel::Left, hs);
    vs.add(vsplitter::Panel::Right, right_pane);
    win.add(vs);
    win.add(bottom_panel);
    win
}

fn main() -> Result<(), appcui::system::Error> {  
    App::single_window(|| {
        one_window_app()
    })
    .theme(ui::theme::hacker_theme())
    .color_schema(false)
    .run()
}