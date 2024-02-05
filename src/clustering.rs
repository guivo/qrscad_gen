#[derive(Clone, Copy)]
pub struct CodeLine {
    pub ix: usize,
    pub iy: usize,
    pub length: usize,
}

/**
 * 
 */
pub fn cluster_lines(elems: Vec<bool>, width: usize) -> Vec<CodeLine> {
    let mut ix: usize = 0usize;
    let mut iy: usize = 0usize;

    let mut active_line: CodeLine = CodeLine { ix: 0usize, iy: 0usize, length: 0usize };

    let mut res: Vec<CodeLine> = Vec::new();

    for val in elems {
        if val && active_line.length==0 {
            active_line.length = 1;
            active_line.ix = ix;
            active_line.iy = iy;
        } else if val {
            active_line.length += 1;
        } else {
            res.push(active_line);
            active_line.length = 0;
        }

        // increment X and Y position (once the last column is reached)
        ix += 1;
        if ix == width {
            if active_line.length>0 {
                res.push(active_line);
                active_line.length = 0;
            }

            ix = 0;
            iy += 1;
        }
    }

    return res;
}
