# QR Code generator

This is yet another QR code generator project. The generatore comes in the
shape of a console command tool, with minimal options. Its unique feature
is the ability to generate the QR code as [OpenSCAD](https://openscad.org/)
files, the OpenSCAD file is a customizable with features offred by the 
software. More standard output format will be added in the future.

The tool is also a [Rust](https://www.rust-lang.org/) excercise. If you want
to compile the code by youself you should download/clone the code and
compile it using the **Cargo** building tool. For more building options
refer to the Rust documentation.

As quick reference, once downladed, the code can be launched the folowing syntax:

```sh
cargo run  -- --text "QR tex content" -o qrscad.scad
```

if the tool is installed in the system the previous syntax can be replaced by:

```sh
qrscad_gen --text "QR tex content" -o qrscad.scad
```

The available options are:

* **outfile**: allow to specify the output OpenSCAD file, default ***qrcode.scad***.
* **text**: is the textual content. If standard code is desired, e.g. WIFI, URL or more,
    the syntax should be manually formatted to encode the wanted message.
* **help**: use this for more options, not documented in this file.

The generated QR code use a **medium** error recovery encoding, fixed within the code,
and the with of the code is the minimal able to hold the textutal content,
with the given error recovery level. While runinng the code swidth is reported.

## 3D print a model

The main use of the tool is to produce an OpenSCAD model, converted into a solid, open
with your preferred slicer and generate a QR code tile though 3D printing.
The best result can be achieved changing filament color at the layer where the code
starts. This can be achived on multi-material printers or asking the slicer to
pause at a given layer. In Prusa Slicer the procedure is automatically suggested
(as 2.4.2), while in Cura a pause can be added modifying the G-code through
the available plugins. In both cases the printer FW shall support the pause
commands.

The size of the tile depends on the QR code width, as number or points. Each
point is horizontally represented by a square of 2 mm width, with an additional frame
by default of 1 point. This means that, without a customization (see next section)
a QR code with size 29 requires 31*2 mm=62 mm wide tile.

## SCAD output features

The file describes  a model with a square base supporting a QR code composed by cubes,
optionally a custom text can be added below the QR code.
The model has by default a fixed size, however global variables can control the size
of the elements, using
the [OpenSCAD Customizer](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Customizer).

Through the customizer it is possible to control the follwing parameters:

* **TileThick**: controls the thickness of the base tile layer.
* **CodeThick**: controls the thickness of the QR code layer.
* **BlockSize**: the QR code is composed by cubes, or blocks, thi parameters control
    the XY size, in mm, of each block. This parameter also controls the size of the 
    tile below the QR code.
* **Frame**: frame surrounding the QR code, as number of equivalent blocks.
* **Note**: if set extrude a text, below the QR code, of the same thickness of
    the QR code.
* **NoteH**: height of the space where the text is placed, if the Note is not empty.
* **NoteOff**: offset of the text baseline.
* **FontSize**: size of the font used for the text.

***Warning** if impossible values are set in the Customizer the solid can be broken.
A Block size too small can result in difficult prints, choose this value
according the nozzle size.
