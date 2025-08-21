use gtk::prelude::*;
use gtk4 as gtk;
use mpris::DBusError;

use crate::{
    musicfinder::{MprisReader, Song},
    user_interface::user_interface::UserInterface,
};

const MAX_SHOWN_LINES: usize = 10;

pub struct GTKInterface {
    shown_lines: usize,
    n_lines_onscreen: usize,
    last_shown_line: String,
    app: gtk::Application,
}

impl UserInterface for GTKInterface {
    fn new() -> Self {
        let application = gtk::Application::builder()
            .application_id("com.lobroc.PlayerLyrics")
            .build();

        application.connect_activate(|app| {
            let tv = gtk::Inscription::builder()
                .wrap_mode(gtk::pango::WrapMode::Word)
                .vexpand(true)
                .build();
            let sw = gtk::ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::Automatic)
                .vscrollbar_policy(gtk::PolicyType::Automatic)
                .child(&tv)
                .build();
            let button = gtk::Button::with_label("Clear text");
            let cover_img = gtk::Image::new();
            cover_img.set_vexpand(true);
            cover_img.set_hexpand(true);

            let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
            content.append(&sw);
            content.append(&cover_img);
            content.append(&button);

            button.connect_clicked(move |_| {
                tv.set_text(Some(""));
            });

            let window = gtk::ApplicationWindow::builder()
                .application(app)
                .title("Player Lyrics")
                .default_width(1280)
                .default_height(720)
                .child(&content)
                .build();

            window.present();
        });
        Self {
            shown_lines: 0,
            n_lines_onscreen: 0,
            last_shown_line: String::new(),
            app: application,
        }
    }

    fn display_song(&self, song_info: &Song, artists: &Vec<String>) -> Result<String, DBusError> {
        let mut playing_status_str: String =
            format!("Now playing: {} by {}", song_info.title, song_info.artist);
        for a in artists[1..].iter() {
            playing_status_str.push_str(&format!("and {}", a));
        }
        self.append_text_to_ui(&playing_status_str);
        Ok(song_info.title.clone())
    }

    fn show_player(&self, mr: &MprisReader) {
        self.append_text_to_ui(format!("Reading from player: {}", mr.get_player_name()).as_str());
    }

    fn display_verse(&mut self, verses: &Vec<&str>, position: usize) {
        for idx in (self.shown_lines as usize)..=position {
            let fetched_verse: &str = match verses.get(idx) {
                Some(s) => s,
                None => {
                    continue;
                }
            };
            if fetched_verse != self.last_shown_line {
                self.append_text_to_ui(fetched_verse);
                self.last_shown_line = fetched_verse.to_string();
            }
        }
        self.shown_lines = position;
    }

    fn clear(&self) {
        let image_container: gtk::Image = self
            .retrieve_content("Image")
            .expect("Image container expected as part of UI")
            .downcast::<gtk::Image>()
            .expect("Downcast expected to work: check done just above.");
        let text_container: gtk::Inscription = self
            .retrieve_content("Inscription")
            .expect("Text container expected as part of UI")
            .downcast::<gtk::Inscription>()
            .expect("Downcast expected to work: check done just above.");
        image_container.clear();
        text_container.set_text(Some(""));
    }
}

impl Clone for GTKInterface {
    fn clone(&self) -> Self {
        Self {
            shown_lines: self.shown_lines, // Copy type
            n_lines_onscreen: self.n_lines_onscreen,
            last_shown_line: self.last_shown_line.clone(),
            app: self.app.clone(),
        }
    }
}

impl GTKInterface {
    pub fn launch(&self) {
        self.app.run();
    }

    pub fn append_text_to_ui(&self, new_text: &str) -> () {
        let text_container: gtk::Inscription = self
            .retrieve_content("Inscription")
            .expect("Text container expected as part of UI")
            .downcast::<gtk::Inscription>()
            .expect("Downcast expected to work: check done just above.");

        let existing_text: String = match text_container.text() {
            Some(t) => t.as_str().to_owned(),
            None => String::from(""),
        };
        let newline_count = existing_text.chars().filter(|&c| c == '\n').count()
            + new_text.chars().filter(|&c| c == '\n').count();
        text_container.set_text(Some((existing_text + "\n" + new_text).as_str()));
        if newline_count >= MAX_SHOWN_LINES {
            self.remove_text_lines_from_ui(newline_count - MAX_SHOWN_LINES + 1);
        }
    }

    pub fn remove_text_lines_from_ui(&self, lines_to_del: usize) -> () {
        let text_container: gtk::Inscription = self
            .retrieve_content("Inscription")
            .expect("Text container expected as part of UI")
            .downcast::<gtk::Inscription>()
            .expect("Downcast expected to work: check done just above.");

        let existing_text: String = match text_container.text() {
            Some(t) => t.as_str().to_owned(),
            None => {
                return;
            }
        };
        let mut new_text_buf = String::new();
        let mut line_iterator = existing_text.split('\n');
        let newline_count = existing_text.chars().filter(|&c| c == '\n').count();
        for idx in 0..newline_count {
            eprintln!(
                "At iteration {} start, we have buffer: {}",
                idx, new_text_buf
            );
            eprintln!("idx: {}", idx);
            eprintln!("2 <= idx: {}", 2 <= idx);
            eprintln!("idx < (2 + lines_to_del) : {}", idx < (2 + lines_to_del));
            if 2 <= idx && idx < (2 + lines_to_del) {
                line_iterator.next();
                eprintln!("Iteration skipped!");
                continue;
            }
            match line_iterator.next() {
                // Consume first two lines, they're the title and lyrics text
                Some(item) => {
                    if item != "" {
                        new_text_buf.push_str(&(String::from(item) + "\n"))
                    }
                }
                None => {
                    continue;
                }
            }
        }
        text_container.set_text(Some(&new_text_buf.trim_end()));
    }

    fn retrieve_content(&self, field_name: &str) -> Option<gtk::Widget> {
        let root_child: gtk4::Widget = self
            .app
            .windows()
            .get(0)
            .expect("Cannot show player: no windows!")
            .child()
            .expect("Empty window is unexpected");
        let mut container: Option<gtk::Widget> = None;

        let mut child_opt = root_child.first_child();
        while let Some(child) = child_opt {
            if let Ok(btn) = child.clone().downcast::<gtk::Button>()
                && field_name == "Button"
            {
                container = Some(btn.upcast::<gtk::Widget>());
            }
            if let Ok(img) = child.clone().downcast::<gtk::Image>()
                && field_name == "Image"
            {
                container = Some(img.upcast::<gtk::Widget>());
            }
            if let Ok(sw) = child.clone().downcast::<gtk::ScrolledWindow>()
                && field_name == "Inscription"
            {
                let mut child_nested_iter = sw.first_child();
                while let Some(nested_child) = child_nested_iter {
                    if let Ok(vp) = nested_child.clone().downcast::<gtk::Viewport>() {
                        let insc = vp.first_child().expect("Expect that there is something in the viewport. Otherwise, it's useless!");
                        container = Some(insc.upcast::<gtk::Widget>());
                    }
                    child_nested_iter = nested_child.next_sibling();
                }
            }

            child_opt = child.next_sibling();
        }
        container
    }

    pub fn display_song(
        &self,
        song_info: &Song,
        artists: &Vec<String>,
        mr: &MprisReader,
    ) -> Result<String, DBusError> {
        let mut playing_status_str: String =
            format!("Now playing: {} by {}", song_info.title, song_info.artist);
        for a in artists[1..].iter() {
            playing_status_str.push_str(&format!("and {}", a));
        }

        if let Some(cover) = mr.get_cover_art() {
            let image_container: gtk::Image = self
                .retrieve_content("Image")
                .expect("Image container expected as part of UI")
                .downcast::<gtk::Image>()
                .expect("Downcast expected to work: check done just above.");
            image_container.set_from_pixbuf(Some(&cover));
        }

        self.append_text_to_ui(&playing_status_str);
        Ok(song_info.title.clone())
    }
}
