use std::{fs::{File, self}, io::Write, path::Path};

use clap::Parser;
use qr_code::{EcLevel, QrCode};

mod clustering;

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
CodeThick = 1.0; // .2
//Unitary size (mm) of the QRCode blocks
BlockSize = 2.0; // .2
Tolerance = .2;
// Fame size
Frame = 1;
// Has not
hasNote = false;
// Line 1
Line1 = \"\";
// Line 2
Line2 = \"\";
// Line 3
Line3 = \"\";
// Height of the note area
NoteH = 10;
// Offset for the text
NoteOff = 3;
// Font type
FontType = \"Segoe UI:style=Bold\";
// Note text size
FontSize = 6;

/* QR Content: {}*/\n\n

nElements = {}+Frame*2; //Tile width\n\n ",
            content, width
        ))
        .unwrap();

    // block associated with the tile layer, supporting the QR code
    scadfile.write_all(b"color(\"white\") {
    translate([0,-nElements*BlockSize,0]) cube([nElements*BlockSize, nElements*BlockSize, TileThick]);
    if (hasNote) {
        h1 = len(Line1)>0 ? NoteH : 0;
        h2 = len(Line2)>0 ? NoteH : 0;
        h3 = len(Line3)>0 ? NoteH : 0;
        
        totH = h1+h2+h3;

        translate([0,-nElements*BlockSize-totH,0]) cube([nElements*BlockSize, totH, TileThick]);
    }
}\n").unwrap();

    // prepare the block of the file associated with the QR code
    scadfile.write_all(b"color(\"black\") {
if (len(Line1)>0 && hasNote) {
    translate([nElements*BlockSize/2, -nElements*BlockSize-NoteH+NoteOff, TileThick-Tolerance]) linear_extrude(CodeThick+Tolerance) text(Line1, FontSize, FontType, halign=\"center\");
}\n").unwrap();
    scadfile.write_all(b"if (len(Line2)>0 && hasNote) {
    translate([nElements*BlockSize/2, -nElements*BlockSize-NoteH*2+NoteOff, TileThick-Tolerance]) linear_extrude(CodeThick+Tolerance) text(Line2, FontSize, FontType, halign=\"center\");
}\n").unwrap();
    scadfile.write_all(b"if (len(Line3)>0 && hasNote) {
    translate([nElements*BlockSize/2, -nElements*BlockSize-NoteH*3+NoteOff, TileThick-Tolerance]) linear_extrude(CodeThick+Tolerance) text(Line3, FontSize, FontType, halign=\"center\");
}\n").unwrap();

    let lines = clustering::cluster_lines(qrcode.to_vec(), width);

    for line in lines {
                
            // for each valid block a line describing a cube with standard X,Y and Z dimensione is printed
        if line.length>0 {
            scadfile.write_fmt(format_args!("  translate([({}+Frame)*BlockSize, -({}+Frame+1)*BlockSize, TileThick-Tolerance]) cube([{}*BlockSize, BlockSize, CodeThick+Tolerance]);\n", line.ix, line.iy, line.length)).unwrap();
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
    #[clap(short, long, value_parser, default_value="")]
    text: String,

    // input file, used if the text is not used
    #[clap(short, long, value_parser, default_value="")]
    input: String,

    // Output file path
    #[clap(short, long, value_parser, default_value = "qrcode.scad")]
    output: String,

    // Error protection level
    #[clap(short, long, value_parser, default_value = "M")]
    eclevel: String,
}

/**
 * Tool entry points
 */
fn main() {
    // parse the command line options
    let args = Args::parse();
    
    // textual content
    let content; 

    if args.input.len()>0 {
        content = fs::read_to_string(args.input).unwrap();
    } else if  args.text.len()>0 {
        content =  args.text;
    } else {
        content = String::from("This is a test!");
    }

    // path of the SCAD file
    let outpath = args.output;

    let eclevel : EcLevel;
    match args.eclevel.as_str() {
        "L" => eclevel = EcLevel::L,
        "M" => eclevel = EcLevel::M,
        "Q" => eclevel = EcLevel::Q,
        "H" => eclevel = EcLevel::H,
        _ => eclevel = EcLevel::M,
    }; 

    // generate the QR code
    let qrcode = QrCode::with_error_correction_level(content.as_bytes(), eclevel).unwrap();

    // Print debug information
    let width = qrcode.width();
    println!("QR Code size={}", width);

    // Save the QR content in a SCAD file
    save_scad(outpath, &qrcode, &content);
}
