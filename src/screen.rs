pub const COLUMN_WIDTH: usize = 80;
pub const ROW_HEIGHT: usize = 25;

use crate::utils::*;

enum ScreenType {
    Display,
    ChooseYesNo,
    ChooseNumber,
    EnterString,
}
impl Default for ScreenType {
    fn default() -> Self {
        ScreenType::Display
    }
}

#[derive(Default)]
pub struct Screen {
    header: String,
    footer_height: u32,
    msg: String,
    screen_type: ScreenType,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            footer_height: 1,
            ..Default::default()
        }
    }

    pub fn get_header(&self) -> String {
        self.header.clone()
    }

    pub fn set_header(&mut self, header: &str) {
        self.header = header.to_string();
    }

    pub fn set_footer_height(&mut self, footer_height: u32) {
        self.footer_height = footer_height;
    }

    pub fn set_msg(&mut self, msg: &str) {
        self.msg = msg.to_string();
    }

    pub fn set_map(&mut self, map_vec: Vec<Vec<char>>, position_x: u32, position_y: u32) {
        //Generate top coorinate of map, may be negative
        //Assume header/footer are one line, cut off later if not
        let msg_area_width = COLUMN_WIDTH;
        let msg_area_height = ROW_HEIGHT as u32
                          - 1 //Header
                          - 1 //Spacer
                          - 1 //Bottom Spacer
                          - 1; //Footer

        let top_pos_x: i32 = position_x as i32 - msg_area_width as i32 / 2;
        let top_pos_y: i32 = position_y as i32 - msg_area_height as i32 / 2;

        let mut msg_string = "".to_string();
        for y in top_pos_y..=top_pos_y + msg_area_height as i32 {
            for x in top_pos_x..top_pos_x + msg_area_width as i32 {
                if y < 0
                    || x < 0
                    || y >= map_vec.len() as i32
                    || x >= map_vec[y as usize].len() as i32
                {
                    msg_string = format!("{}X", msg_string);
                } else {
                    //Draw Pat player position
                    if x as u32 == position_x && y as u32 == position_y {
                        msg_string = format!("{}{}", msg_string, 'P');
                    } else {
                        msg_string = format!("{}{}", msg_string, map_vec[y as usize][x as usize]);
                    }
                }
            }
            msg_string = format!("{}\n", msg_string);
        }
        self.msg = msg_string.trim().to_string();
    }

    pub fn draw_display(&mut self) {
        self.screen_type = ScreenType::Display;
        self.set_footer_height(1);
        self.draw();
        pause();
    }

    pub fn draw_pick_yes_or_no(&mut self, msg: &str) -> bool {
        self.screen_type = ScreenType::ChooseYesNo;
        self.set_footer_height(count_newlines(&msg));
        self.draw();
        pick_yes_or_no(&msg)
    }

    pub fn draw_pick_a_number(&mut self, msg: &str, low: u32, high: u32) -> u32 {
        self.screen_type = ScreenType::ChooseNumber;
        self.set_footer_height(count_newlines(&msg));
        self.draw();
        pick_number(&msg, low, high)
    }

    pub fn draw_enter_string(&mut self, msg: &str) -> String {
        self.screen_type = ScreenType::EnterString;
        self.set_footer_height(count_newlines(&msg));
        self.draw();
        enter_string(&msg)
    }

    pub fn draw_enter_char(&mut self, msg: &str) -> char {
        self.screen_type = ScreenType::EnterString;
        self.set_footer_height(count_newlines(&msg));
        self.draw();
        enter_char(&msg)
    }

    fn draw(&self) {
        let mut position = 0;
        let header_formatted = textwrap::fill(
            &self.header,
            textwrap::Options::new(COLUMN_WIDTH)
                .initial_indent("")
                .subsequent_indent(""),
        );

        let header_linecount = count_newlines(&header_formatted);
        let msg_formatted = textwrap::fill(
            &self.msg,
            textwrap::Options::new(COLUMN_WIDTH)
                .initial_indent("")
                .subsequent_indent(""),
        );
        let msg_formatted_vec: Vec<String> =
            msg_formatted.split('\n').map(|s| s.to_string()).collect();

        let msg_linecount = count_newlines(&msg_formatted);

        let msg_area = ROW_HEIGHT as u32
                          - header_linecount //Header
                          - 1 //Spacer
                          - 1 //Bottom Spacer
                          - self.footer_height; //Footer

        //While the message is too big for the current area, need to do scrolling
        while msg_linecount - position > msg_area {
            clear();

            println!("{}", header_formatted);

            println!("{}", "-".repeat(COLUMN_WIDTH));

            for n in 0..msg_area - 1 {
                println!("{}", msg_formatted_vec[(n + position) as usize]);
            }
            position += (msg_area * 3) / 4;

            println!("...");
            println!("{}", "-".repeat(COLUMN_WIDTH));
            pause();
        }

        //At this point the message can fit into the current area, just print and add spacing
        clear();

        println!("{}", header_formatted);

        println!("{}", "-".repeat(COLUMN_WIDTH));

        for n in 0..(msg_formatted_vec.len() - position as usize) {
            println!("{}", msg_formatted_vec[n + position as usize]);
        }
        if msg_linecount - position < msg_area {
            for _ in 1..(msg_area + position - msg_linecount) {
                println!(".");
            }
        }

        println!("{}", "-".repeat(COLUMN_WIDTH));
    }
}
