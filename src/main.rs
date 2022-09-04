use std::{fs::File, io::Write, path::Path};

use clap::Parser;
use qr_code::QrCode;

/**
 * Save the content of the QR code associated to the
 * given content into a SCAD file.
 * 
 * The SCAD file is a template that allows to be customized
 * using the standard Customizer tool.
 * 
 * The code produce a panic error if any problem is encountered
 * file saving the content into the file.
 */
fn save_scad(outpath: String, qrcode: &QrCode, content: &String) {
    // Open the file associated to the given path
    let scadpath = Path::new(outpath.as_str());
    let display = scadpath.display();

    let mut scadfile = match File::create(&scadpath) {
        Ok(scadfile) => scadfile,
        Err(why) => panic!("Error creating {}, {}", display, why),
    };

    // collect the width of the QR code, no surrounding frame.
    let width = qrcode.width();

    // Print the SCAD file header
    scadfile
        .write_fmt(format_args!(
            "//Thickness (mm) of the tile layer
TileThick = 2.0; // .2
//Thickness (mm) of QRCode layer
CodeThick = 2.0; // .2
//Unitary size (mm) of the QRCode blocks
BlockSize = 2.0; // .2
// Fame size
Frame = 1;
// Text note added to the tile, if empty ignored
Note = \"\";
// Height of the note area
NoteH = 10;
// Offset for the text
NoteOff = 3;
// Note text size
FontSize = 6;

/* QR Content: {}*/\n\n

nElements = {}+Frame*2; //Tile width\n\n ",
            content, width
        ))
        .unwrap();

    // index used to monitor the position in the output grid
    let mut ix = 0usize;
    let mut iy = 0usize;

    // block associated with the tile layer, supporting the QR code
    scadfile.write_all(b"color(\"white\") {
    translate([0,-nElements*BlockSize,0]) cube([nElements*BlockSize, nElements*BlockSize, TileThick]);
    if (len(Note)>0) {
        translate([0,-nElements*BlockSize-10,0]) cube([nElements*BlockSize, NoteH, TileThick]);
    }
}\n").unwrap();

    // prepare the block of the file associated with the QR code
    scadfile.write_all(b"color(\"black\") {
if (len(Note)>0) {
    translate([nElements*BlockSize/2, -nElements*BlockSize-NoteH+NoteOff, TileThick]) linear_extrude(CodeThick) text(Note, FontSize, halign=\"center\");
}\n").unwrap();
    let elems = qrcode.to_vec();
    for val in elems {
        if val {
            // for each valid block a line describing a cube with standard X,Y and Z dimensione is printed
            scadfile.write_fmt(format_args!("  translate([({}+Frame)*BlockSize, -({}+Frame+1)*BlockSize, TileThick]) cube([BlockSize, BlockSize, CodeThick]);\n", ix, iy)).unwrap();
        }

        // increment X and Y position (once the last column is reached)
        ix += 1;
        if ix == width {
            ix = 0;
            iy += 1;
        }
    }

    // close the QR code block
    scadfile.write_all(b"}\n").unwrap();

    // ensure all content is flushed from the internal buffers.
    scadfile.flush().unwrap();
}

/**
 * Define the structure associated to the CLI options
 * using CLAP annotations..
 */
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    // Text content to be code in the QR
    #[clap(short, long, value_parser)]
    text: String,

    // Output file path
    #[clap(short, long, value_parser, default_value = "qrcode.scad")]
    outfile: String,
}

/**
 * Tool entry points
 */
fn main() {
    // parse the command line options
    let args = Args::parse();

    // textual content
    let content = args.text; 
    // path of the SCAD file
    let outpath = args.outfile;

    // generate the QR code
    //TODO: check for errors
    let qrcode = QrCode::new(content.as_bytes()).unwrap();

    // Print debug information
    let width = qrcode.width();
    println!("QR Code size={}", width);

    // Save the QR content in a SCAD file
    save_scad(outpath, &qrcode, &content);
}
