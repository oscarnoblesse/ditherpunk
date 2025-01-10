use argh::FromArgs;
use image::{GenericImageView, ImageError};
use image::DynamicImage;
#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {

    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    // la deuxieme couleur d'entre
    #[argh(positional)]
    nombre_de_couleur: String,


    // le fichier de sortie
    #[argh(positional)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    PixelBlanc(OptsPixelBlanc),
    DualColorMix(OptsDualColorMix),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="pixelBlanc")]
/// Rendu de l’image en mode pixel blanc.
struct OptsPixelBlanc {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="dualColorMix")]
/// Rendu de l’image en mode pixel blanc.
struct OptsDualColorMix {}

 
const WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
const GREY: image::Rgb<u8> = image::Rgb([127, 127, 127]);
const BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);
const BLUE: image::Rgb<u8> = image::Rgb([0, 0, 255]);
const RED: image::Rgb<u8> = image::Rgb([255, 0, 0]);
const GREEN: image::Rgb<u8> = image::Rgb([0, 255, 0]);
const YELLOW: image::Rgb<u8> = image::Rgb([255, 255, 0]);
const MAGENTA: image::Rgb<u8> = image::Rgb([255, 0, 255]);
const CYAN: image::Rgb<u8> = image::Rgb([0, 255, 255]);


fn main() -> Result<(), ImageError> {
    let args: DitherArgs = argh::from_env();
    let img = image::open(&args.input)?; 

    match args.mode {
        Mode::Seuil(_) => {
            let _ = mode_seuil(img, &args.output);
        }

        Mode::PixelBlanc(_) => {
            let _ = mode_pixel_blanc(img, &args.output);
        }

        Mode::DualColorMix(_) => {
            let _ = mode_dual_couleur_pallete(img, &args.output , &args.nombre_de_couleur );
        }
    }

    Ok(())
}


fn save_image(img: DynamicImage, output: &Option<String>) -> Result<(), ImageError> {
    match output {
        Some(path) => img.save(path)?,
        None => println!("Image sauvegardée sans spécifier de nom.")
    }
    Ok(())
}


fn mode_seuil(img: DynamicImage, output: &Option<String>) -> Result<(), ImageError> {
    let img_bw = img.grayscale().to_luma8(); 
    let img_bw = image::ImageBuffer::from_fn(img_bw.width(), img_bw.height(), |x, y| {
        let pixel = img_bw.get_pixel(x, y);
        if pixel[0] > 128 { 
            image::Luma([255]) 
        } else {
            image::Luma([0]) 
        }
    });
    let img_bw = DynamicImage::ImageLuma8(img_bw).to_rgb8();
    let pixel = img_bw.get_pixel(32, 50);
    println!("Pixel(32, 50): {:?}", pixel);
    let _ = save_image(DynamicImage::ImageRgb8(img_bw), output)?;
    Ok(())
}


fn mode_pixel_blanc(img: DynamicImage, output: &Option<String>) {
    let mut img_rgb = img.to_rgb8();
    let (width, height) = img_rgb.dimensions();

    for y in 0..height {
        for x in 0..width {
            if (x + y) % 2 == 0 {
                img_rgb.put_pixel(x, y, WHITE);
            }
        }
    }
    let _ = save_image(DynamicImage::ImageRgb8(img_rgb), output);
}


fn color_distance(pixel: &image::Rgba<u8>, color: &image::Rgb<u8>) -> i32 {
    let r_diff = pixel[0] as i32 - color[0] as i32;
    let g_diff = pixel[1] as i32 - color[1] as i32;
    let b_diff = pixel[2] as i32 - color[2] as i32;
    r_diff * r_diff + g_diff * g_diff + b_diff * b_diff
}

fn trouver_la_couleur_la_plus_proche(pixel: &image::Rgba<u8>, palette: &[image::Rgb<u8>]) -> image::Rgb<u8> {
    let mut min_distance = i32::MAX;
    let mut closest_color = palette[0];

    for &color in palette.iter() {
        let distance = color_distance(pixel, &color);
        if distance < min_distance {
            min_distance = distance;
            closest_color = color;
        }
    }

    closest_color
}
fn mode_dual_couleur_pallete(img: DynamicImage, output_path: &Option<String>, nombre_de_la_pallette: &str) -> Result<(), image::ImageError> {
    let (width, height) = img.dimensions();
    let mut new_img = img.to_rgb8(); 
    let palette = get_color_palette(nombre_de_la_pallette);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let closest_color = trouver_la_couleur_la_plus_proche(&pixel, &palette);
            new_img.put_pixel(x, y, closest_color);
        }
    }

    let _ = save_image(DynamicImage::ImageRgb8(new_img), output_path);
    Ok(())
}


fn get_color_palette(nombre_de_la_pallette: &str) -> Vec<image::Rgb<u8>> {
    let colors = vec![WHITE, BLACK, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA,GREY];
    let mut palette = Vec::new();
    let n: usize = nombre_de_la_pallette.parse().unwrap_or(2).clamp(2, 8);

    for i in 0..n {
        palette.push(colors[i]);
    }

    palette
}