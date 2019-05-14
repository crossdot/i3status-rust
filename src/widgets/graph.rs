use crate::config::Config;
use crate::widget::State;
use super::super::widget::I3BarWidget;
use super::super::widget::Properties;
use num::{clamp, ToPrimitive};

#[derive(Clone, Debug)]
pub struct GraphWidget {
    content: Option<String>,
    icon: Option<String>,
    state: State,
    rendered: Properties,
    cached_output: Option<String>,
    config: Config,
}
#[allow(dead_code)]
impl GraphWidget {
    pub fn new(config: Config) -> Self {
        GraphWidget {
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

    pub fn with_state(mut self, state: State) -> Self {
        self.state = state;
        self.update();
        self
    }

    pub fn set_values<T>(&mut self, content: &[T], min: Option<T>, max: Option<T>)
    where
        T: Ord + ToPrimitive,
    {
        let bars = ["_", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
        let min: f64 = match min {
            Some(x) => x.to_f64().unwrap(),
            None => content.iter().min().unwrap().to_f64().unwrap(),
        };
        let max: f64 = match max {
            Some(x) => x.to_f64().unwrap(),
            None => content.iter().max().unwrap().to_f64().unwrap(),
        };
        let extant = max - min;
        if extant.is_normal() {
            let length = bars.len() as f64 - 1.0;
            let bar = content
                .iter()
                .map(|x| {
                    bars[((clamp(x.to_f64().unwrap(), min, max) - min) / extant * length) as usize]
                })
                .collect::<Vec<&'static str>>()
                .concat();
            self.content = Some(bar);
        } else {
            let bar = (0..content.len() - 1)
                .map(|_| bars[0])
                .collect::<Vec<&'static str>>()
                .concat();
            self.content = Some(bar);
        }
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
        self.rendered.color = key_fg.to_owned();
        self.rendered.background = key_bg.to_owned();

        // self.cached_output = Some(self.rendered.to_string());
    }
}

impl I3BarWidget for GraphWidget {
    fn to_string(&self) -> String {
        // format!("{}{} ",    self.icon.clone().unwrap_or_else(|| String::from(" ")),
                            // self.content.clone().unwrap_or_else(|| String::from("")))
        
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
