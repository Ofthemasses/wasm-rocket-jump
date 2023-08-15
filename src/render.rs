use web_sys::console;

#[derive(Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8
}


pub struct Display {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display{
        Display{
            pixels: vec![0; width*height*4],
            width,
            height
        }
    }

    pub fn clear(&mut self){
        self.pixels = vec![0; self.width*self.height*4];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: Colour){
        if x < self.width && y < self.height {
            let index = (x + y * self.width) * 4;
            self.pixels[index] = c.r;
            self.pixels[index + 1] = c.g;
            self.pixels[index + 2] = c.b;
            self.pixels[index + 3] = 255;
        }
    }

    pub fn draw_triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, c: Colour){
        self.draw_line(x1, y1, x2, y2, c.clone());
        self.draw_line(x2, y2, x3, y3, c.clone());
        self.draw_line(x3, y3, x1, y1, c.clone());
    }

    pub fn fill_triangle(&mut self, mut x1: f32, mut y1: f32, mut x2: f32, mut y2: f32, mut x3: f32, mut y3:f32, c: Colour){
        // Sort vertices by y-coordinate (top to bottom)
        let mut vertices = [(x1, y1), (x2, y2), (x3, y3)];

        vertices.sort_unstable_by(|(_, y1), (_, y2)| y1.partial_cmp(y2).unwrap());

        let (sorted_x1, sorted_y1) = vertices[0];
        let (sorted_x2, sorted_y2) = vertices[1];
        let (sorted_x3, sorted_y3) = vertices[2];

        // Calculate slopes
        let slope1 = (sorted_x3 - sorted_x1) / (sorted_y3 - sorted_y1);
        let slope2 = (sorted_x2 - sorted_x1) / (sorted_y2 - sorted_y1);
        let slope3 = (sorted_x3 - sorted_x2) / (sorted_y3 - sorted_y2);

        // Initialize current x coordinates
        let mut curr_x1 = sorted_x1;
        let mut curr_x2 = sorted_x1;

        // Scanline filling
        for y in sorted_y1 as usize..=sorted_y3 as usize {
            let start_x = (curr_x1 + (y as f32 - sorted_y1) * slope1).max(0.0).min(self.width as f32 - 1.0).ceil() as usize;
            let end_x = (curr_x2 + (y as f32 - sorted_y1) * slope2).max(0.0).min(self.width as f32 - 1.0).floor() as usize;

            for x in start_x..=end_x {
                self.set_pixel(x, y, c.clone());
            }

            curr_x1 += slope1;
            curr_x2 += slope2;

            if y == sorted_y2 as usize {
                curr_x1 = sorted_x2;
                curr_x2 += slope3;
            }
        }
    }

    pub fn draw_line(&mut self, mut x1: f32, mut y1: f32, x2: f32, y2: f32, c: Colour){
        let dx: f32 = (x2 - x1).abs();
        let sx: f32 = if x1 < x2 {1.0} else {-1.0};
        let dy: f32 = -(y2 - y1).abs();
        let sy: f32 = if y1 < y2 { 1.0 } else { -1.0 };
        let mut error: f32 = dx + dy;
        let mut e2: f32;

        loop {
            self.set_pixel((x1-1.0) as usize, (y1-1.0) as usize, c.clone());
            if x1.round() == x2.round() && y1.round() == y2.round() {
                break;
            }
            e2 = 2.0 * error;
            if e2 >= dy {
                if x1.round() == x2.round() { break; }
                error = error + dy;
                x1 = x1 + sx;
            }
            if e2 <= dx {
                if y1.round() == y2.round() { break; }
                error = error + dx;
                y1 = y1 + sy;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Clone, Copy)]
pub struct Tri {
    pub points: [Point; 3]
}

impl Tri {
    // Consider deleting after testing purposes
    pub fn new(p1x: f32, p1y: f32, p1z: f32, p2x: f32, p2y: f32, p2z: f32, p3x: f32, p3y: f32, p3z: f32) -> Tri{
        Tri{
            points: [Point{x:p1x,y:p1y,z:p1z},Point{x:p2x,y:p2y,z:p2z},Point{x:p3x,y:p3y,z:p3z}]
        }
    }
}

// Turn into array and create mutMesh? `pb fn new (size: u32) -> mesh)
pub struct Mesh {
    pub tris: Vec<Tri>
}

pub fn multiply_matrix_vector(i: Point, o: &mut Point, m: [[f32; 4]; 4]){
    o.x = i.x * m[0][0] + i.y * m[1][0] + i.z * m[2][0] + m[3][0];
    o.y = i.x * m[0][1] + i.y * m[1][1] + i.z * m[2][1] + m[3][1];
    o.z = i.x * m[0][2] + i.y * m[1][2] + i.z * m[2][2] + m[3][2];
    let w: f32 = i.x * m[0][3] + i.y * m[1][3]+ i.z * m[2][3] + m[3][3];

    if w != 0.0 {
        o.x /= w;
        o.y /= w;
        o.z /= w;
    }
}