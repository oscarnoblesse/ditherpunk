use argh::FromArgs;
use image::{GenericImageView, ImageError};
use image::DynamicImage;
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
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {

    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
}
 
// const WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
// const GREY: image::Rgb<u8> = image::Rgb([127, 127, 127]);
// const BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);
// const BLUE: image::Rgb<u8> = image::Rgb([0, 0, 255]);
// const RED: image::Rgb<u8> = image::Rgb([255, 0, 0]);
// const GREEN: image::Rgb<u8> = image::Rgb([0, 255, 0]);
// const YELLOW: image::Rgb<u8> = image::Rgb([255, 255, 0]);
// const MAGENTA: image::Rgb<u8> = image::Rgb([255, 0, 255]);
// const CYAN: image::Rgb<u8> = image::Rgb([0, 255, 255]);


fn main() -> Result<(), ImageError> {
    let args: DitherArgs = argh::from_env();
    let img = image::open(&args.input)?; 

    match args.mode {
        Mode::Seuil(_) => {
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
            save_image(DynamicImage::ImageRgb8(img_bw), &args.output)?;
        }
        Mode::Palette(opts_palette) => {
            let mut img_palette = img.to_rgb8(); 
            img_palette = reduce_palette(img_palette, opts_palette.n_couleurs);
            save_image(DynamicImage::ImageRgb8(img_palette), &args.output)?;
        }
    }

    Ok(())
}

fn reduce_palette(img: image::RgbaImage, n_couleurs: usize) -> image::RgbaImage {
    // Implémentation de réduction de palette ici...
    img
}

fn save_image(img: DynamicImage, output: &Option<String>) -> Result<(), ImageError> {
    match output {
        Some(path) => img.save(path)?,
        None => println!("Image sauvegardée sans spécifier de nom.")
    }
    Ok(())
}