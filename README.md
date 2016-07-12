# termui

Termui is a cross-platform library written in rust to make simple console GUIs.

Note: It's still in a _very_ early stage of development.

# Requirements

A nightly build of the rust compiler, found [here](https://www.rust-lang.org/en-US/downloads.html).

# Compiling

Just type

```
cd code_directory
```
and then
```
cargo build
```

# Usage

Here is a small example using termui

```rust
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
```

# Expanding

If you want to expand the library with your new graphical elements you only need to make them implement the trait ```UiElement``` (found in the module ```gelements```), which is the following:

```rust
pub trait UiElement {
    fn render (&self) -> String;
    fn get_props (&self) -> &UiProps;
}
```

Where render is called upon rendering and get_props is used to get general proprieties of the object, such as margin styling.

Look at ```textarea.rs``` to find an example of a simple element.
