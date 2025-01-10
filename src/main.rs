use argh::FromArgs;
use image::{GenericImageView, ImageError, DynamicImage, ImageBuffer};

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {

    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

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
    SeuilNoirBlanc(OptsSeuilNoirBlanc),

}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "seuilNoirBlanc", description = "mode de seuillage aux choix")]
struct OptsSeuilNoirBlanc {
    /// Couleur pour les pixels noirs (format: R,G,B)
    #[argh(option, description = "mouleur pour les pixels noirs (format: R,G,B)")]
    noir: String,

    /// Couleur pour les pixels blancs (format: R,G,B)
    #[argh(option, description = "mouleur pour les pixels blancs (format: R,G,B)")]
    blanc: String,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "pixelBlanc", description = "mode de pixel blanc")]
struct OptsPixelBlanc {
}

#[derive(Debug, Clone, PartialEq, FromArgs)]

#[argh(subcommand, name="dualColorMix")]
/// Rendu de l’image en mode pixel blanc.
struct OptsDualColorMix {
    /// Couleur pour les pixels blancs (format: R,G,B)
    #[argh(option, default = "String::from(\"0\")", description = "mouleur pour les pixels blancs (format: R,G,B)")]
    nombre_palette: String,
}

 
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

        Mode::DualColorMix(opts_dual_color_mix) => {
            let _ = mode_dual_couleur_pallete(img, &args.output , opts_dual_color_mix.nombre_palette );
        }
        Mode::SeuilNoirBlanc(opts_seuil) => {
            let _ = mode_seuil_noir_blanc(img, &args.output, opts_seuil);
        }

    }

    Ok(())
}


fn get_pixel_luminosity(pixel: image::Rgb<u8>) -> u8 {
    // Pondération selon les coefficients standards pour le calcul de luminance
    let red = pixel[0] as f32 * 0.2126;
    let green = pixel[1] as f32 * 0.7152;
    let blue = pixel[2] as f32 * 0.0722;

    // Calculer la luminance et la convertir en entier
    (red + green + blue).round() as u8
}

fn save_image(img: DynamicImage, output: &Option<String>) -> Result<(), ImageError> {
    match output {
        Some(path) => img.save(path)?,
        None => println!("Image sauvegardée sans spécifier de nom.")
    }
    Ok(())
}


fn mode_seuil(img: DynamicImage, output: &Option<String>) -> Result<(), ImageError> {
    let grayscale = img.grayscale().to_rgb8();
    
    let thresholded = ImageBuffer::from_fn(grayscale.width(), grayscale.height(), |x, y| {
        let pixel = grayscale.get_pixel(x, y);
        let luminosity = get_pixel_luminosity(*pixel);
        if luminosity > 128 {
            image::Rgb([255, 255, 255])
        } else {
            image::Rgb([0, 0, 0])
        }
    });
    
    let rgb_image = DynamicImage::ImageRgb8(thresholded);
    
    let pixel_value = rgb_image.get_pixel(32, 50);
    println!("Pixel (32, 50): {:?}", pixel_value);
    
    save_image(rgb_image, output)?;
    
    Ok(())
}


fn mode_seuil_noir_blanc(img: DynamicImage, output: &Option<String>, opts: OptsSeuilNoirBlanc) -> Result<(), ImageError> {
    let grayscale = img.grayscale().to_rgb8(); 

    let noir: Vec<u8> = opts.noir.split(',').map(|s| s.parse().unwrap()).collect();
    let blanc: Vec<u8> = opts.blanc.split(',').map(|s| s.parse().unwrap()).collect();

    let thresholded = ImageBuffer::from_fn(grayscale.width(), grayscale.height(), |x, y| {
        let pixel = grayscale.get_pixel(x, y);
        let luminosity = get_pixel_luminosity(*pixel);
        if luminosity > 128 { 
            image::Rgb([blanc[0], blanc[1], blanc[2]]) 
        } else {
            image::Rgb([noir[0], noir[1], noir[2]]) 
        }
    });

    let rgb_image = DynamicImage::ImageRgb8(thresholded);
    let pixel = rgb_image.get_pixel(32, 50);
    println!("Pixel(32, 50): {:?}", pixel);
    save_image(rgb_image, output)?;
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
fn mode_dual_couleur_pallete(img: DynamicImage, output_path: &Option<String>, nombre_de_la_pallette: String) -> Result<(), image::ImageError> {
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


fn get_color_palette(nombre_de_la_pallette: String) -> Vec<image::Rgb<u8>> {
    let colors = vec![WHITE, BLACK, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA,GREY];
    let mut palette = Vec::new();
    let n: usize = nombre_de_la_pallette.parse().unwrap_or(2).clamp(2, 8);

    for i in 0..n {
        palette.push(colors[i]);
    }

    palette
}