# QR Code generator

This is yet another QR code generator project. The generatoe comes in the
shape of a light command tool, with minimal options. A unique feature
is the ability to generate the QR code as [OpenSCAD](https://openscad.org/)
file. More standard output format will be added.

The tool is also a [Rust](https://www.rust-lang.org/) excercise. If you want
to compile the code by youself you should download/clone the code and
compile it using the **Cargo** building tool.

Once compiled the code accept the folowing syntax:

```sh
cargo run  -- --text "QR tex content" -o qrscad.scad
```

if the tool was installed in the system the previous syntax can be replaced by:

```sh
qrscad_gen --text "QR tex content" -o qrscad.scad
```

The options are:

* **outfile**: allow to specify the output OpenSCAD file, default ***qrcode.scad***.
* **text**: is the textual content. If standard code is desired, e.g. WIFI, URL or more,
    the syntax should be manually formatted to encode the wanted message.

The generated QR code use a **medium** error recovery encoding and the with of the code
is the minimal able to hold the textutal content, with the given error recovery level.
While runinng the code swidth is reported.

## 3D print a model

The main use of the tool is to produce a SCAD model, converted into a solid, open
with your preferred slicer and generate a QR code tile though 3D printing. 
The best result can be achieved changing filament color at the layer where the code
starts. This can be achived on multi-material printers or asking the slicer to
pause at a given layer. In Prusa Slicer the procedure is automatically suggested 
(as 2.4.2).

The default parameters are enough to produce a printable model on most printers, the
pause GCode command shall also be supported.

The size of the tile depends on the QR code width, as number or points. Each
point is horizontally represented by a square of 2 mm width, with an additional frame
by default of 1 point. This means that, without a customization (see next section)
a QR code with size 29 requires 31*2 mm=62 mm wide tile.

## SCAD output features

The file contains a model with a square base with the QR code composed by cubes.
The tool doesn't allow to control the sizes, because the model is written to exploit
the [OpenSCAD Customizer](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Customizer).

Through the customizer it is possible to control the follwing parameters:

* **TileThick**: controls the thickness of the base tile layer.
* **CodeThick**: controls the thickness of the QR code layer.
* **BlockSize**: the QR code is composed by cubes, or blocks, thi parameters control
    the XY size, in mm, of each block. This parameter also controls the size of the 
    tile below the QR code.
* **Frame**: frame surrounding the QR code, as number of equivalent blocks.

***Warning** if impossible values are set in the Customizer the solid can be broken.
A Block size too small can result in difficult prints, choose this value
according the nozzle size.
