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

pub fn get_element_height (rendered: String) -> i32 {
    let mut height = 0;
    for ch in rendered.chars() {
        if ch == '\n' {
            height += 1;
        }
    }
    
    height
}

pub fn get_element_width (rendered: String) -> i32 {
    let mut max_line_len = 0;
    let mut line_len = 0;

    for (i, ch) in rendered.chars().enumerate() {
        line_len += 1;
        if ch == '\n' || i+1 == rendered.len() {
            if line_len > max_line_len {
                max_line_len = line_len;
                
                if ch == '\n' {
                    max_line_len -= 1; // -1 because it counted the '\n'
                }
            }
            
            line_len = 0;
        }
    }
    
    max_line_len
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