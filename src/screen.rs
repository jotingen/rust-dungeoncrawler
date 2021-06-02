pub const COLUMN_WIDTH: usize = 80;
pub const ROW_HEIGHT: usize = 25;

use crate::utils::*;
use crossterm::{
    cursor::{Hide, MoveTo},
    ExecutableCommand,
};
use std::io::{stdout, Write};

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
    footer: String,
    msg: String,
    screen_type: ScreenType,
    buffer: Vec<Vec<char>>,
}

impl Screen {
    pub fn new() -> Screen {
        clear();
        stdout().execute(Hide).unwrap();
        stdout().flush().unwrap();
        Screen {
            buffer: vec![vec![' '; COLUMN_WIDTH]; ROW_HEIGHT],
            ..Default::default()
        }
    }

    pub fn get_header(&self) -> String {
        self.header.clone()
    }

    pub fn set_header(
        &mut self,
        header: &str,
    ) {
        self.header = header.to_string();
    }

    pub fn set_footer(
        &mut self,
        footer: &str,
    ) {
        self.footer = footer.to_string();
    }

    pub fn set_msg(
        &mut self,
        msg: &str,
    ) {
        self.msg = msg.to_string();
    }

    pub fn set_map(
        &mut self,
        map_vec: Vec<Vec<char>>,
        position_x: i32,
        position_y: i32,
    ) {
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
        for y in top_pos_y..top_pos_y + msg_area_height as i32 {
            for x in top_pos_x..top_pos_x + msg_area_width as i32 {
                if y < 0
                    || x < 0
                    || y >= map_vec.len() as i32
                    || x >= map_vec[y as usize].len() as i32
                {
                    msg_string = format!("{} ", msg_string);
                } else {
                    //Draw Pat player position
                    if x == position_x && y == position_y {
                        msg_string = format!("{}{}", msg_string, '@');
                    } else {
                        msg_string = format!("{}{}", msg_string, map_vec[y as usize][x as usize]);
                    }
                }
            }
            msg_string = format!("{}\n", msg_string);
        }
        msg_string.pop(); //Remove trailing newline
        self.msg = msg_string;
    }

    pub fn draw_display(&mut self) {
        self.screen_type = ScreenType::Display;
        self.set_footer("Press Enter to continue...");
        self.draw();
        pause();
    }

    pub fn draw_pick_yes_or_no(
        &mut self,
        msg: &str,
    ) -> bool {
        self.screen_type = ScreenType::ChooseYesNo;
        self.set_footer(&format!("{} (Y/n)",&msg));
        self.draw();
        pick_yes_or_no()
    }

    pub fn draw_pick_a_number(
        &mut self,
        msg: &str,
        low: u32,
        high: u32,
    ) -> u32 {
        self.screen_type = ScreenType::ChooseNumber;
        self.set_footer(&format!("{} {}-{} ",&msg, low, high));
        self.draw();
        pick_number(low, high)
    }

    pub fn draw_enter_string(
        &mut self,
        msg: &str,
    ) -> String {
        self.screen_type = ScreenType::EnterString;
        self.set_footer(&msg);
        self.draw();
        enter_string()
    }

    pub fn draw_enter_char(
        &mut self,
        msg: &str,
    ) -> char {
        self.screen_type = ScreenType::EnterString;
        self.set_footer(&msg);
        self.draw();
        enter_char()
    }

    fn draw(&mut self) {
        let mut position = 0;
        let header_formatted = textwrap::fill(
            &self.header,
            textwrap::Options::new(COLUMN_WIDTH)
                .initial_indent("")
                .subsequent_indent(""),
        );
        let header_formatted_vec: Vec<String> = header_formatted
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        let header_linecount = count_newlines(&header_formatted);

        let msg_formatted = strip_trailing_newline(&textwrap::fill(
            &self.msg,
            textwrap::Options::new(COLUMN_WIDTH)
                .initial_indent("")
                .subsequent_indent(""),
        ))
        .to_string();
        let msg_formatted_vec: Vec<String> =
            msg_formatted.split('\n').map(|s| s.to_string()).collect();
        let msg_linecount = count_newlines(&msg_formatted);
        
        let footer_formatted = textwrap::fill(
            &self.footer,
            textwrap::Options::new(COLUMN_WIDTH)
                .initial_indent("")
                .subsequent_indent(""),
        );
        let footer_formatted_vec: Vec<String> = footer_formatted
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        let footer_linecount = count_newlines(&footer_formatted);

        let msg_area = ROW_HEIGHT as u32
                          - header_linecount //Header
                          - 1 //Spacer
                          - 1 //Bottom Spacer
                          - footer_linecount; //Footer

        //While the message is too big for the current area, need to do scrolling
        let mut buffer_new: Vec<Vec<char>>;
        let mut buffer_column;
        while msg_linecount - position > msg_area {
            buffer_new = vec![vec![' '; COLUMN_WIDTH]; ROW_HEIGHT];
            buffer_column = 0;

            for header_formatted in header_formatted_vec.iter() {
                buffer_new[buffer_column]
                    .splice(0..header_formatted.len(), header_formatted.chars());
                buffer_column += 1;
            }

            buffer_new[buffer_column] = vec!['-'; COLUMN_WIDTH];
            buffer_column += 1;

            for n in 0..msg_area - 1 {
                buffer_new[buffer_column].splice(
                    0..msg_formatted_vec[(n + position) as usize].len(),
                    msg_formatted_vec[(n + position) as usize].chars(),
                );
                buffer_column += 1;
            }
            position += (msg_area * 3) / 4;

            buffer_new[buffer_column].splice(0..3, vec!['.'; 3]);

            buffer_column = ROW_HEIGHT - 2;
            buffer_new[buffer_column] = vec!['-'; COLUMN_WIDTH];

            #[allow(clippy::clippy::needless_range_loop)]
            for row in 0..ROW_HEIGHT {
                for col in 0..COLUMN_WIDTH {
                    if self.buffer[row][col] != buffer_new[row][col] {
                        self.buffer[row][col] = buffer_new[row][col];
                        stdout().execute(MoveTo(col as u16, row as u16)).unwrap();
                        stdout().write_all(&[self.buffer[row][col] as u8]).unwrap();
                    }
                }
            }
            stdout()
                .execute(MoveTo(0, (ROW_HEIGHT - 1) as u16))
                .unwrap();
            stdout().flush().unwrap();

            pause();
        }

        //At this point the message can fit into the current area, just print and add spacing
        buffer_new = vec![vec![' '; COLUMN_WIDTH]; ROW_HEIGHT];
        buffer_column = 0;

        for header_formatted in header_formatted_vec.iter() {
            buffer_new[buffer_column].splice(0..header_formatted.len(), header_formatted.chars());
            buffer_column += 1;
        }

        buffer_new[buffer_column] = vec!['-'; COLUMN_WIDTH];
        buffer_column += 1;

        for n in 0..(msg_formatted_vec.len() - position as usize) {
            buffer_new[buffer_column].splice(
                0..msg_formatted_vec[n + position as usize].len(),
                msg_formatted_vec[n + position as usize].chars(),
            );
            buffer_column += 1;
        }

        buffer_column = ROW_HEIGHT - 2;
        buffer_new[buffer_column] = vec!['-'; COLUMN_WIDTH];

        buffer_column = ROW_HEIGHT - 1;
        for footer_formatted in footer_formatted_vec.iter() {
            buffer_new[buffer_column].splice(0..footer_formatted.len(), footer_formatted.chars());
            buffer_column += 1;
        }

        #[allow(clippy::clippy::needless_range_loop)]
        for row in 0..ROW_HEIGHT {
            for col in 0..COLUMN_WIDTH {
                if self.buffer[row][col] != buffer_new[row][col] {
                    self.buffer[row][col] = buffer_new[row][col];
                    stdout().execute(MoveTo(col as u16, row as u16)).unwrap();
                    stdout().write_all(&[self.buffer[row][col] as u8]).unwrap();
                }
            }
        }
        stdout()
            .execute(MoveTo(0, (ROW_HEIGHT - 1) as u16))
            .unwrap();
        stdout().execute(Hide).unwrap();
        stdout().flush().unwrap();
    }
}
