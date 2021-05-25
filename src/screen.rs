const COLUMN_WIDTH: usize = 80;
const ROW_HEIGHT: usize = 25;

use crate::utils::*;

#[derive(Default)]
pub struct Screen {
    header: String,
    footer_height: u32,
    msg: String,
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

    pub fn draw(&self) {
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
