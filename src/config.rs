use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub window_dimensions: (u32, u32),
    pub render_dimensions: (u32, u32),
}

impl Config {
    fn parse_option(&mut self, option: &str, value: &str) {
        match option {
            "window_width" => {
                let val: u32 = value.parse().expect("window width is NAN");
                self.window_dimensions.0 = val;
            }
            "window_height" => {
                let val: u32 = value.parse().expect("window height is NAN");
                self.window_dimensions.1 = val;
            }
            "render_width" => {
                let val: u32 = value.parse().expect("render width is NAN");
                self.render_dimensions.0 = val;
            }
            "render_height" => {
                let val: u32 = value.parse().expect("render height is NAN");
                self.render_dimensions.1 = val;
            }
            _ => println!("Invalid option: {} with value: {}", option, value),
        }

    }

    pub fn read_config(filename: &str) -> Config {
        let mut file = File::open(filename).expect("Error opening config file");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(
            "Error reading config file",
        );

        let mut cfg = Config {
            window_dimensions: (0, 0),
            render_dimensions: (0, 0),
        };

        // match on option=value
        let re = Regex::new(r"(\w+)=(\w+)").unwrap();
        for rematch in re.captures_iter(&contents) {
            cfg.parse_option(&rematch[1], &rematch[2]);
        }

        cfg
    }
}