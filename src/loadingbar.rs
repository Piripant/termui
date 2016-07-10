use gelements::UiElement;
use gelements::UiProps;
use std;

pub struct LoadingBar where LoadingBar : UiElement {
    pub props: UiProps,
    
    pub show_percent: bool,
    percent: i32,
    pub length: i32,
}

impl UiElement for LoadingBar {
    fn render (&self) -> String {
        let mut output = std::iter::repeat("#").take((self.length*self.percent/100) as usize).collect::<String>();
        output.push_str(&std::iter::repeat("-").take((self.length-self.length*self.percent/100) as usize).collect::<String>());
        if self.show_percent {
            output.push_str(&(" ".to_string() + &self.percent.to_string() + "%"));
        }
        
        output
    }
    
    fn get_props (&self) -> &UiProps {
        &self.props
    }
}

impl LoadingBar {
    pub fn new (name: &str, length: i32) -> LoadingBar {
        LoadingBar {
            props: UiProps::new(name.to_string()),
            show_percent: true,
            percent: 0,
            length: length,
        }
    }
    
    pub fn set_percent (&mut self, percent: i32) {
        let mut percent = percent;
        if percent > 100 {
            percent = 100;
        }
        
        self.percent = percent;
    }
    
    pub fn get_precent (&self) -> i32 {
        self.percent
    }
}