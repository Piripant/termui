use gelements::UiElement;
use gelements::UiProps;

pub struct TextArea where TextArea : UiElement {
    pub props: UiProps,
    
    pub text: String,
}

impl UiElement for TextArea {
    fn render (&self) -> String {
        self.text.clone()
    }
    
    fn get_props (&self) -> &UiProps {
        &self.props
    }
}

impl TextArea {
    pub fn new (name: &str, text: String) -> TextArea {
        TextArea {
            props: UiProps::new(name.to_string()),
            text: text,
        }
    }
}