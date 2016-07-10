#![feature(box_syntax)]

extern crate termui;

use termui::window;
use termui::combobox;
use termui::textarea;
use termui::loadingbar;
use termui::gelements;
use termui::gelements::UiElement;

fn main () {
    let mut combo1 = combobox::ComboBox::new("Calettamento");
    let mut text1 = textarea::TextArea::new("Descrizione", "Comunque Lorenzo e' Mendosa De Piava\n".to_string());
    let mut loading1 = loadingbar::LoadingBar::new("Download", 32);
    
    combo1.add_option("Muovi la caletta caletta!");
    combo1.add_option("Muovi la caletta caletta!");
    combo1.add_option("Muovi la caletta caletta!");
    combo1.add_option("Ohhhhh macaletta!");
    
    loading1.props.upper_margin.enabled = false;
    
    loading1.length = gelements::get_element_width(text1.render());
    loading1.show_percent = false;
    loading1.set_percent(75);
    
    window::render(vec![&combo1, &text1, &loading1]);
}

//let text: &mut textarea::TextArea = match text1.as_any().downcast_mut::<&mut textarea::TextArea>() {
//    Some(text) => text,
//    None => panic!("Error casting")
//};