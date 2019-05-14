use crate::themes::Theme;
use serde_json::value::Value;

#[derive(Debug, Clone)]
pub struct Properties {
    pub icon: String,
    pub full_text: String,
    pub separator: bool,
    pub separator_block_width: usize,
    pub background: String,
    pub color: String,
    pub markup: String,
}

#[derive(Debug, Copy, Clone)]
pub enum State {
    Idle,
    Info,
    Good,
    Warning,
    Critical,
}

impl State {
    pub fn theme_keys(self, theme: &Theme) -> (&String, &String) {
        use self::State::*;
        match self {
            Idle => (&theme.idle_bg, &theme.idle_fg),
            Info => (&theme.info_bg, &theme.info_fg),
            Good => (&theme.good_bg, &theme.good_fg),
            Warning => (&theme.warning_bg, &theme.warning_fg),
            Critical => (&theme.critical_bg, &theme.critical_fg),
        }
    }
}

pub trait I3BarWidget {
    fn to_string(&self) -> String;
    fn get_rendered(&self) -> &Properties;
}
