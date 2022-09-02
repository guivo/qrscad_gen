use std::{fs::File, io::Write, path::Path};

use clap::Parser;
use qr_code::QrCode;

fn save_scad(outpath: String, qrcode: &QrCode, content: &String) {
    let scadpath = Path::new(outpath.as_str());
    let display = scadpath.display();

    let mut scadfile = match File::create(&scadpath) {
        Ok(scadfile) => scadfile,
        Err(why) => panic!("Error creating {}, {}", display, why),
    };

    let width = qrcode.width();

    // Print the SCAD file header
    scadfile
        .write_fmt(format_args!(
            "//Thickness (mm) of the base and QRCode part
BaseSize = 2.0; // .2
//Unitary size (mm) of the QRCode blocks
BlockSize = 2.0; // .2
// Number of blocks for the bottom box
nElements = {};
// Offset
Offset = 0;

// QR Content: {}\n\n",
            width + 2,
            content
        ))
        .unwrap();

    let mut ix = 0usize;
    let mut iy = 0usize;

    scadfile.write_all(b"color(\"white\") translate([0,-nElements*BlockSize,0]) cube([nElements*BlockSize, nElements*BlockSize, BaseSize]);\n").unwrap();

    // write the header file
    scadfile.write_all(b"color(\"black\") {\n").unwrap();
    let elems = qrcode.to_vec();
    for val in elems {
        if val {
            scadfile.write_fmt(format_args!("  translate([({}+Offset)*BlockSize, -({}+Offset+1)*BlockSize, BaseSize]) cube([BlockSize, BlockSize, BaseSize]);\n", ix+1, iy+1)).unwrap();
        }
        ix += 1;
        if ix == width {
            ix = 0;
            iy += 1;
        }
    }

    scadfile.write_all(b"}\n").unwrap();

    scadfile.flush().unwrap();
}

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

fn main() {
    let args = Args::parse();

    let content = args.text;
    let outpath = args.outfile;

    let qrcode = QrCode::new(content.as_bytes()).unwrap();

    let width = qrcode.width();
    println!("QR Code size={}", width);

    save_scad(outpath, &qrcode, &content);
}
