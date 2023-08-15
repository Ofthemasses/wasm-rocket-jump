
mod render;

use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use crate::render::*;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    //let mut data = context
    //    .create_image_data_with_sw_and_sh(512.0,512.0)
    //    .unwrap()
    //    .dyn_into::<web_sys::ImageData>()
    //    .unwrap();
    
    let mut disp = Display::new(1920,1080);
    let mesh_cube: Mesh;
    let camera: Point = Point{
        x: 0.0,
        y: 0.0,
        z: 0.0
    };
    mesh_cube = Mesh {
        tris: vec![

            // SOUTH
            Tri::new(0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0),
            Tri::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0),

            // EAST
            Tri::new(1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0),
            Tri::new(1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0),

            // NORTH
            Tri::new(1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0),
            Tri::new(1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0),

            // WEST
            Tri::new(0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0),
            Tri::new(0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0),

            // TOP
            Tri::new(0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0),
            Tri::new(0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0),

            // BOTTOM
            Tri::new(1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
            Tri::new(1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
        ]
    };

    // Projection Matrix
    let f_near: f32 = 0.1;
    let f_far: f32 = 1000.0;
    let f_fov: f32 = 90.0;
    let f_aspect_ratio: f32 = disp.height as f32 / disp.width as f32;
    let f_fov_rad = 1.0 / (f_fov * 0.5 / 180.0 * PI).tan();

    let matrix_projection: [[f32; 4]; 4] = [
        [f_aspect_ratio*f_fov_rad,0.0,0.0,0.0],
        [0.0,f_fov_rad,0.0,0.0],
        [0.0,0.0,f_far / (f_far - f_near),1.0],
        [0.0,0.0,(-f_far*f_near)/(f_far-f_near),0.0]
    ];

    let mut matrix_rot_z: [[f32; 4]; 4] = [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ];
    let mut matrix_rot_x: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ];
    let mut f_theta: f32 = 0.0;

    use web_sys::console;
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut time_elapsed: f64 = 0.0;
    *g.borrow_mut() = Some(Closure::new(move |time_stamp: f64| {
        f_theta += 0.001 * (time_elapsed - time_stamp) as f32;
        time_elapsed = time_stamp;
        // Rotation Z;
        matrix_rot_z[0][0] = f_theta.cos();
        matrix_rot_z[0][1] = f_theta.sin();
        matrix_rot_z[1][0] = -f_theta.sin();
        matrix_rot_z[1][1] = f_theta.cos();

        // Rotation X
        matrix_rot_x[1][1] = (f_theta * 0.5).cos();
        matrix_rot_x[1][2] = (f_theta * 0.5).sin();
        matrix_rot_x[2][1] = -(f_theta * 0.5).sin();
        matrix_rot_x[2][2] = (f_theta * 0.5).cos();

        for tri in &mesh_cube.tris {
            let mut tri_proj: Tri = Tri { points: [Point{x: 0.0, y: 0.0, z: 0.0}; 3] };
            let mut tri_rotated_z: Tri = tri_proj.clone();
            let mut tri_rotated_zx: Tri = tri_proj.clone();
            multiply_matrix_vector(tri.points[0],  &mut tri_rotated_z.points[0], matrix_rot_z);
            multiply_matrix_vector(tri.points[1], &mut tri_rotated_z.points[1], matrix_rot_z);
            multiply_matrix_vector(tri.points[2], &mut tri_rotated_z.points[2], matrix_rot_z);

            multiply_matrix_vector(tri_rotated_z.points[0],  &mut tri_rotated_zx.points[0], matrix_rot_x);
            multiply_matrix_vector(tri_rotated_z.points[1], &mut tri_rotated_zx.points[1], matrix_rot_x);
            multiply_matrix_vector(tri_rotated_z.points[2], &mut tri_rotated_zx.points[2], matrix_rot_x);

            // Offset into the screen
            let mut tri_translated: Tri = tri_rotated_zx;
            tri_translated.points[0].z += 3.0;
            tri_translated.points[1].z += 3.0;
            tri_translated.points[2].z += 3.0;

            let mut normal: Point = Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let mut line1: Point = Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let mut line2: Point = Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            line1.x = tri_translated.points[1].x - tri_translated.points[0].x;
            line1.y = tri_translated.points[1].y - tri_translated.points[0].y;
            line1.z = tri_translated.points[1].z - tri_translated.points[0].z;

            line2.x = tri_translated.points[2].x - tri_translated.points[0].x;
            line2.y = tri_translated.points[2].y - tri_translated.points[0].y;
            line2.z = tri_translated.points[2].z - tri_translated.points[0].z;

            normal.x = line1.y * line2.z - line1.z * line2.y;
            normal.y = line1.z * line2.x - line1.x * line2.z;
            normal.z = line1.x * line2.y - line1.y * line2.x;

            let l: f32 = (normal.x*normal.x + normal.y*normal.y + normal.z*normal.z).sqrt();
            normal.x /= l; normal.y /= l; normal.z /= l;

            if  normal.x * (tri_translated.points[0].x - camera.x) +
                normal.x * (tri_translated.points[0].y - camera.y) +
                normal.z * (tri_translated.points[0].z - camera.z) < 0.0
            {
            multiply_matrix_vector(tri_translated.points[0],  &mut tri_proj.points[0], matrix_projection);
            multiply_matrix_vector(tri_translated.points[1], &mut tri_proj.points[1], matrix_projection);
            multiply_matrix_vector(tri_translated.points[2], &mut tri_proj.points[2], matrix_projection);

            // Scale into view
            tri_proj.points[0].x += 1.0;
            tri_proj.points[0].y += 1.0;
            tri_proj.points[1].x += 1.0;
            tri_proj.points[1].y += 1.0;
            tri_proj.points[2].x += 1.0;
            tri_proj.points[2].y += 1.0;

            tri_proj.points[0].x *= 0.5 * disp.width as f32;
            tri_proj.points[0].y *= 0.5 * disp.height as f32;
            tri_proj.points[1].x *= 0.5 * disp.width as f32;
            tri_proj.points[1].y *= 0.5 * disp.height as f32;
            tri_proj.points[2].x *= 0.5 * disp.width as f32;
            tri_proj.points[2].y *= 0.5 * disp.height as f32;



            disp.draw_triangle(
               tri_proj.points[0].x, tri_proj.points[0].y,
               tri_proj.points[1].x, tri_proj.points[1].y,
               tri_proj.points[2].x, tri_proj.points[2].y,
               Colour { r: 0, g: 0, b: 0 }
            );
            }
        }
        // let mut i: usize = 0;
        //  let mut d = data.data();
        //  ci = 0;
        // for col in &disp.pixels {
        //
        //     console::log_1(&ci.into());
        //     ci += 1;
        //     d[i] = col.r;
        //     d[i+1] = col.g;
        //     d[i+2] = col.b;
        //     d[i+3] = 255;
        //     i += 4;
        // }
        let put_res = context.put_image_data(&web_sys::ImageData::new_with_u8_clamped_array_and_sh(
               Clamped(&mut disp.pixels.clone()),
               disp.width as u32,
               disp.height as u32
           ).unwrap(), 0.0, 0.0);
        if put_res.is_err() {
            console::log_1(&put_res.err().unwrap());
            let _ = f.borrow_mut().take();
            return;
        }
        disp.clear();
        request_animation_frame(f.borrow().as_ref().unwrap());
     }));
    request_animation_frame(g.borrow().as_ref().unwrap());
}


