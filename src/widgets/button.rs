use crate::config::Config;
use crate::widget::State;
use super::super::widget::I3BarWidget;
use super::super::widget::Properties;

#[derive(Clone, Debug)]
pub struct ButtonWidget {
    content: Option<String>,
    icon: Option<String>,
    state: State,
    id: String,
    rendered: Properties,
    cached_output: Option<String>,
    config: Config,
}

impl ButtonWidget {
    pub fn new(config: Config, id: &str) -> Self {
        ButtonWidget {
            content: None,
            icon: None,
            state: State::Idle,
            id: String::from(id),
            rendered: Properties {
                icon: "".to_owned(),
                full_text: "".to_owned(),
                separator: false,
                separator_block_width: 0,
                background: "#000000".to_owned(),
                color: "#000000".to_owned(),
                markup: "pango".to_owned(),
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

    pub fn with_content(mut self, content: Option<String>) -> Self {
        self.content = content;
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

    pub fn set_text<S: Into<String>>(&mut self, content: S) {
        self.content = Some(content.into());
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

        self.rendered.full_text = self.content.clone().unwrap_or_else(|| String::from(""));
        self.rendered.icon = self.icon.clone().unwrap_or_else(|| String::from(" "));
        self.rendered.color = key_fg.to_string();
        self.rendered.background = key_bg.to_string();

        // self.cached_output = Some(self.rendered.to_string());
    }
}

impl I3BarWidget for ButtonWidget {
    fn to_string(&self) -> String {
        let (key_bg, key_fg) = self.state.theme_keys(&self.config.theme);

        format!("<action=`source ~/.profile ; NAME={} mouse.sh` button=1><action=`source ~/.profile ; BUTTON=2 NAME={} mouse.sh` button=2><action=`source ~/.profile ; BUTTON=3 NAME={} mouse.sh` button=3><action=`source ~/.profile ; BUTTON=4 NAME={} mouse.sh` button=4><action=`source ~/.profile ; BUTTON=5 NAME={} mouse.sh` button=5><fc={},{}><fn=1>{}</fn>{}</fc></action></action></action></action></action>",
            self.id.to_owned(),
            self.id.to_owned(),
            self.id.to_owned(),
            self.id.to_owned(),
            self.id.to_owned(),
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
