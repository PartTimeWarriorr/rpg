use std::{collections::HashMap};
use std::{fs, path};
use ggez::{graphics::Image, Context, GameResult};

pub struct Assets {
    pub character_images: HashMap<String, Image>,
}


impl Assets {
    pub fn new(ctx: &Context) -> GameResult<Assets> {

        let mut map : HashMap<String, Image> = HashMap::new();

        // let character_image = Image::from_path(ctx, "/char_1.png")?;

        let paths = fs::read_dir("./resources").unwrap();
        for path in paths {

            let path_name = String::from(path.unwrap().path().to_str().unwrap().strip_prefix("./resources").unwrap());
            let image = Image::from_path(ctx, path_name.clone())?;
            let image_name= String::from(path_name.strip_prefix("\\").unwrap().strip_suffix(".png").unwrap());
            map.insert(image_name, image);
        }
        // let character_image = Image::from_path(ctx, "/char_1.png")?;
        dbg!(&map.keys());

        Ok(
            Assets {
                // character_image
                character_images: map
            }
        )
    }
}