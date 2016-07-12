#![feature(box_syntax)]

extern crate termui;

use termui::window;
use termui::combobox;
use termui::textarea;

fn main () {
    let mut text1 = textarea::TextArea::new("Description title:", "Here are some options! \n".to_string());
    let mut combo1 = combobox::ComboBox::new("A title which will never be displayed :(");

    combo1.props.show_title = false;
    
    text1.props.bottom_margin.enabled = false;
    combo1.props.upper_margin.enabled = false;

    combo1.add_option("A first option");
    combo1.add_option("A second option");
    combo1.add_option("Even a third option!");

    window::render(vec![&text1, &combo1]);
}

//let text: &mut textarea::TextArea = match text1.as_any().downcast_mut::<&mut textarea::TextArea>() {
//    Some(text) => text,
//    None => panic!("Error casting")
//};