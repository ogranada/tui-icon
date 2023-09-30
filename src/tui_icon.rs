use std::fs;
use tui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

type TColor = (u8, u8, u8);
type TOffset = (usize, usize);


#[derive(Debug, Clone, Copy)]
pub struct IconColor {
    is_reset: bool,
    values: TColor,
}

impl IconColor {
    pub fn default() -> Self {
        IconColor {
            is_reset: true,
            values: (0, 0, 0),
        }
    }
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        IconColor {
            is_reset: false,
            values: (r, g, b),
        }
    }
    pub fn as_color(&self) -> Color {
        if self.is_reset {
            Color::Reset
        } else {
            Color::Rgb(self.values.0, self.values.1, self.values.2)
        }
    }
}

#[derive(Debug)]
pub struct Icon {
    _path: String,
    _length: usize,
    offset: TOffset,
    default_color: Option<IconColor>,
    format_version: String,
    contents: Vec<u8>,
    c_width: usize,
    pub width: usize,
    pub height: usize,
}

impl Icon {
    pub fn new(file_path: &str, default_color: Option<IconColor>, offset: Option<TOffset>) -> Self {
        let contents: Vec<u8> = fs::read(&file_path).expect(format!("Invalid file path '{}'.", file_path).as_str());
        let csize = contents.len();
        let mut cntr = 0;
        let mut to = None;
        let width;
        let height;
        let data_start;
        let mut format_version = "unknown".to_string();
        loop {
            let (line, new_to) = Icon::read_line(&contents, to);
            to = new_to;
            if !line.starts_with("#") {
                match cntr {
                    0 => {
                        format_version = line;
                    }
                    1 => {
                        let (_width, _height) = Icon::get_dimentions(line);
                        width = _width;
                        height = _height;
                        data_start = to.unwrap();
                        break;
                    }
                    _ => {}
                };
                cntr += 1;
            }
        }
        let new_content = &contents[data_start..csize];
        let _length = new_content.len();
        let c_width = ((width / 8) + (if width % 8 > 0 { 1 } else { 0 })) * 8;
        Icon {
            _path: file_path.to_string(),
            _length,
            offset: offset.unwrap_or((0, 0)),
            default_color,
            format_version,
            contents: new_content.to_owned(),
            c_width,
            width,
            height,
        }
    }

    fn read_line(contents: &Vec<u8>, start: Option<usize>) -> (String, Option<usize>) {
        let mut cntr = start.unwrap_or(0);
        let mut chr = contents[cntr] as char;
        let mut result = String::from("");
        while !chr.eq(&'\n') {
            result = format!("{}{}", result, chr);
            cntr += 1;
            chr = contents[cntr] as char;
        }
        (result, Some(cntr + 1))
    }

    fn get_dimentions(line: String) -> (usize, usize) {
        let data: Vec<usize> = line
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .map(|n| n as usize)
            .collect();
        (data[0], data[1])
    }

    /*
    pub fn print_ascii(&self) {
        let data: Vec<String> = self
            .contents
            .iter()
            .map(|val| format!("{:0>8}", format!("{:b}", val)))
            .collect();
        let data = data.concat();
        println!("{}", data.clone());
    }
    // */

    fn prepare_p4_matrix(&self, color: IconColor) -> Vec<Option<(usize, usize, IconColor)>> {
        let data: Vec<String> = self
            .contents
            .iter()
            .map(|val| format!("{:0>8}", format!("{:b}", val)))
            .collect();
        let data = data.concat();
        let data: Vec<char> = data.chars().collect();
        let mut c: char;
        let mut res = vec![];
        for j in 0..self.height {
            for i in 0..self.c_width {
                let pos = j * self.c_width + i;
                c = data[pos];
                res.push(if !c.eq(&'0') {
                    Some((i + self.offset.0, j + self.offset.1, color))
                } else {
                    None
                });
            }
        }
        res
    }

    pub fn get_icon_matrix(&self) -> Vec<Option<(usize, usize, IconColor)>> {
        let color = self.default_color.unwrap_or(IconColor::default());
        match self.format_version.as_str() {
            "P4" => self.prepare_p4_matrix(color),
            _ => {
                vec![]
            }
        }
    }
}

impl Shape for Icon {
    fn draw(&self, painter: &mut Painter) {
        let data = self.get_icon_matrix();
        for item in data {
            if let Some((i, j, c)) = item {
                painter.paint(i, j, c.as_color());
            }
        }
    }
}
