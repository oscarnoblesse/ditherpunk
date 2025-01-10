use argh::FromArgs;
use image::{GenericImageView, ImageError, DynamicImage, ImageBuffer, Luma};

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {

    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
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
    SeuilNoirBlanc(OptsSeuilNoirBlanc),
    Palette(OptsPalette),
    pixelBlanc(OptsPixelBlanc),
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
struct OptsPixelBlanc {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "palette", description = "mode de palette de couleurs")]
struct OptsPalette {
    /// Nombre de couleurs dans la palette
    #[argh(option, description = "nombre de couleurs dans la palette")]
    n_couleurs: usize
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
            mode_seuil(img, &args.output);
        }
        Mode::SeuilNoirBlanc(opts_seuil) => {
            mode_seuilNoirBlanc(img, &args.output, opts_seuil);
        }
        Mode::Palette(opts_palette) => {
           mode_palette(opts_palette, img, &args.output);
        }
        Mode::pixelBlanc(_) => {
            mode_pixelBlanc(img, &args.output);
        }
    }

    Ok(())
}

fn reduce_palette(img: image::RgbImage, n_couleurs: usize) -> image::RgbImage {
    img
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

fn mode_palette(opts_palette: OptsPalette, img: DynamicImage, output: &Option<String>) {
    let mut img_palette = img.to_rgb8(); 
    img_palette = reduce_palette(img_palette, opts_palette.n_couleurs);
    save_image(DynamicImage::ImageRgb8(img_palette), output).unwrap();
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


fn mode_seuilNoirBlanc(img: DynamicImage, output: &Option<String>, opts: OptsSeuilNoirBlanc) -> Result<(), ImageError> {
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


fn mode_pixelBlanc(img: DynamicImage, output: &Option<String>) {
    let mut img_rgb = img.to_rgb8();
    let (width, height) = img_rgb.dimensions();

    for y in 0..height {
        for x in 0..width {
            if (x + y) % 2 == 0 {
                img_rgb.put_pixel(x, y, WHITE);
            }
        }
    }

    save_image(DynamicImage::ImageRgb8(img_rgb), output);
    
}
