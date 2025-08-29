use std::fs::File;
use std::io::BufReader;
use std::{collections::HashMap};
use std::{fs, path};
use ggez::{graphics::Image, Context, GameResult};

use serde::{Serialize, Deserialize};
use serde_json::{Value};

pub struct Assets {
    pub images: HashMap<String, Image>,
}

impl Assets {
    pub fn new(ctx: &Context) -> GameResult<Assets> {

        let mut map : HashMap<String, Image> = HashMap::new();

        let paths = fs::read_dir("./resources").unwrap();
        for path in paths {

            let path_name = String::from(path.unwrap().path().to_str().unwrap().strip_prefix("./resources").unwrap());
            let image = Image::from_path(ctx, path_name.clone())?;
            let image_name= String::from(path_name.strip_prefix("\\").unwrap().strip_suffix(".png").unwrap());
            map.insert(image_name, image);
        }

        Ok(
            Assets {
                images: map
            }
        )
    }
}