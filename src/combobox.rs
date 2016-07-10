use gelements::UiElement;
use gelements::UiProps;

pub struct ComboBox where ComboBox : UiElement {
    pub props: UiProps,
    
    pub show_nums: bool,
    options: Vec<String>,
}

impl UiElement for ComboBox {
    fn render (&self) -> String {
        let mut output: String = "".to_string();
        for (i, opt) in self.options.iter().enumerate() {
            let mut opt_str = opt.clone();
            
            if i < self.options.len()-1 {
                opt_str = opt_str + "\n";
            }
            
            if self.show_nums {
                opt_str = (i+1).to_string() + ") " + &opt_str;
            }
            
            output.push_str(&opt_str);
        }
        
        output
    }
    
    fn get_props (&self) -> &UiProps {
        &self.props
    }
}

impl ComboBox {
    pub fn new (name: &str) -> ComboBox {
        ComboBox {
            show_nums: true,
            props: UiProps::new(name.to_string()),
            options: Vec::new()
        }
    }
    
    pub fn add_option (&mut self, text: &str) {
        self.options.push(text.to_string());
    }
    
    pub fn remove_option (&mut self, id: usize) {
        self.options.remove(id);
    }
}