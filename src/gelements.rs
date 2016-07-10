pub struct UiProps {
    pub name: String,
    pub show_name: bool,
    pub right_margin: Margin,
    pub left_margin: Margin,
    pub bottom_margin: Margin,
    pub upper_margin: Margin,
}

impl UiProps {
    pub fn new (name: String) -> UiProps {
        UiProps {
            name: name,
            show_name: true,
            right_margin: Margin::new(),
            left_margin: Margin::new(),
            bottom_margin: Margin::new(),
            upper_margin: Margin::new(),
        }
    }
}

pub trait UiElement {
    fn render (&self) -> String;
    fn get_props (&self) -> &UiProps;
}

pub struct Margin {
    pub enabled: bool,
    pub width: i32,
    pub space_between: i32,
}

impl Margin {
    pub fn new () -> Margin {
        Margin {
            enabled: true,
            width: 1,
            space_between: 0
        }
    }
}