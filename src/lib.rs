use std::{error::Error, path::Path};
use image::{io::Reader, DynamicImage, RgbImage, ImageResult};

pub fn load_image<Q>(path: Q) -> Result<DynamicImage, Box<dyn Error>>
    where Q: AsRef<Path>,
{
    Ok(Reader::open(path)?.decode()?)
}

pub fn save_image<Q>(path: Q, img: &DynamicImage) -> ImageResult<()>
    where Q: AsRef<Path>,
{
    img.save(path)?;
    Ok(())
}

pub fn trim(img: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>> {
    let img = img.to_rgb8();

    let (width, height) = img.dimensions();

    let mut minx = width;
    let mut maxx = 0u32;
    let mut miny = height;
    let mut maxy = 0u32;

    for x in 0..width {
        for y in 0..height {
            let p = img.get_pixel(x, y);
            if p.0 != [255, 255, 255] {
                minx = minx.min(x);
                maxx = maxx.max(x);
                miny = miny.min(y);
                maxy = maxy.max(y);
            }
        }
    }

    if minx > maxx || miny > maxy {
        Err("err")?;
    }

    let rwidth = maxx - minx + 1;
    let rheight = maxy - miny + 1;

    let mut rimg = RgbImage::new(rwidth, rheight);
    for x in 0..rwidth {
        for y in 0..rheight {
            rimg.put_pixel(x, y, *img.get_pixel(x + minx, y + miny));
        }
    }
    
    Ok(DynamicImage::ImageRgb8(rimg))
}