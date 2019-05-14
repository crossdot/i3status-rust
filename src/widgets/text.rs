use crate::config::Config;
use crate::widget::State;
use super::super::widget::I3BarWidget;
use super::super::widget::Properties;

#[derive(Clone, Debug)]
pub struct TextWidget {
    content: Option<String>,
    icon: Option<String>,
    state: State,
    rendered: Properties,
    cached_output: Option<String>,
    config: Config,
}

impl TextWidget {
    pub fn new(config: Config) -> Self {
        TextWidget {
            content: None,
            icon: None,
            state: State::Idle,
            rendered: Properties {
                icon: "".to_owned(),
                full_text: "".to_owned(),
                separator: false,
                separator_block_width: 0,
                background: "#000000".to_owned(),
                color: "#000000".to_owned(),
                markup: "".to_owned(),
            },
            config,
            cached_output: None,
        }
    }

    pub fn with_icon(mut self, name: &str) -> Self {
        self.icon = self.config.icons.get(name).cloned();
        self.update();
        self
    }

    pub fn with_text(mut self, content: &str) -> Self {
        self.content = Some(String::from(content));
        self.update();
        self
    }

    pub fn with_state(mut self, state: State) -> Self {
        self.state = state;
        self.update();
        self
    }

    pub fn set_text(&mut self, content: String) {
        self.content = Some(content);
        self.update();
    }

    pub fn set_icon(&mut self, name: &str) {
        self.icon = self.config.icons.get(name).cloned();
        self.update();
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
        self.update();
    }

    fn update(&mut self) {
        let (key_bg, key_fg) = self.state.theme_keys(&self.config.theme);

        self.rendered.icon = self.icon.clone().unwrap_or_else(|| String::from(" "));
        self.rendered.full_text = self.content.clone().unwrap_or_else(|| String::from(""));
        self.rendered.color = key_fg.to_owned();
        self.rendered.background = key_bg.to_owned();

        // self.cached_output = Some(self.rendered.to_string());
    }
}

impl I3BarWidget for TextWidget {
    fn to_string(&self) -> String {
        // format!("{}{} ", self.icon.clone().unwrap_or_else(|| String::from(" ")), self.content.clone().unwrap_or_else(|| String::from("")))

        format!("<fc={},{}><fn=1>{}</fn> {}</fc>", 
            self.rendered.color[..7].to_owned(),
            self.rendered.background[..7].to_owned(),
            self.rendered.icon,
            self.rendered.full_text
        )
    }

    fn get_rendered(&self) -> &Properties {
        &self.rendered
    }
}
