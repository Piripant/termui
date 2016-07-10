use gelements::UiElement;
use std;

pub fn clear () {
    print!("{}[2J", 27 as char);
}

pub fn render (elements: Vec<&UiElement>) {
    clear();
    
    for el in elements {
        let mut to_render = el.render();
        
        if to_render.len() > 0 {
            if el.get_props().show_name {
                to_render = el.get_props().name.clone() + ":\n" + &to_render.clone();
            }
            
            add_margins(el, &mut to_render);
            
            print!("{}\n", to_render);
        }
    }
}

fn add_margins (element: &UiElement, to_render: &mut String) {
    let props = element.get_props();
    
    // Vertical Margins
    
    // Right Margin
    if props.right_margin.enabled {
        let mut lastch = ' ';
        let mut to_ren = "".to_string();
        to_ren.push_str("| ");
        
        for ch in to_render.chars() {
            if lastch == '\n' {
                to_ren.push_str("| ");
            }
            
            to_ren.push(ch);
            lastch = ch;
        }
        
        *to_render = to_ren;
    }
    
    // Left Margin
    if props.left_margin.enabled {
        let mut to_ren = "".to_string();
        let mut max_line_len = 0;
        let mut line_len = 0;
        
        for (i, ch) in to_render.chars().enumerate() {
            line_len += 1;
            if ch == '\n' || i+1 == to_render.len() {
                if line_len > max_line_len {
                    max_line_len = line_len;
                    
                    if ch == '\n' {
                        max_line_len -= 1; // -1 because it counted the '\n'
                    }
                }
                
                line_len = 0;
            }
        }
        
        line_len = 0;
        for (i, ch) in to_render.chars().enumerate() {
            line_len += 1;
            
            let next_out_bound = i+1 == to_render.len();
            if ch == '\n' || next_out_bound {
                let mut chars_to_place = max_line_len-line_len;
                
                if ch == '\n' {
                    chars_to_place += 1; // It needs one more space
                }
                
                if next_out_bound {
                    to_ren.push(ch);
                }
                
                // Places spaces until the max line length is reached
                for _ in 0..chars_to_place {
                    to_ren.push(' ');
                }
                
                to_ren.push_str(" |");
                line_len = 0;
            }
            
            if !next_out_bound {
                to_ren.push(ch);
            }
        }

        *to_render = to_ren;
    }
    
    // Horizontal margins
    
    let mut m_length = 0; // Margin length
    let mut line_len = 0; // Current line lenght 32
    for (i, ch) in to_render.chars().enumerate() {
        line_len += 1;
        
        // The next char is \n or is the end of to_render
        if ch == '\n' || i+1 == to_render.len() {
            if m_length < line_len {
                m_length = line_len;
                
                if ch == '\n' {
                    m_length -= 1; // -1 because it counted the '\n'
                }
            }
            
            line_len = 0;
        }
    }
    let h_margin = std::iter::repeat("─").take(m_length).collect::<String>();
    
    // Upper Margin
    if props.upper_margin.enabled {
        *to_render = h_margin.clone() + "\n" + &to_render.clone();
    }
    
    // Bottom Margin
    if props.bottom_margin.enabled {
        *to_render = to_render.clone() + "\n" + &h_margin.clone();
    }
}





