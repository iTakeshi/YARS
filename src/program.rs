use glium;
use glium::backend::glutin_backend::GlutinFacade;

use std::fs::File;
use std::io::BufReader;

use obj::{load_obj, Vertex};

fn get_reader(path: &str) -> BufReader<File> {
    use std::error::Error;
    use std::path::Path;

    let path = Path::new(&path);
    match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => BufReader::new(file),
    }
}

fn read_file(path: &str) -> String {
    use std::error::Error;
    use std::io::Read;

    let mut s = String::new();
    match get_reader(&path).read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path, why.description()),
        Ok(_) => s,
    }
}

pub fn axis_program(display: &GlutinFacade) -> glium::Program {
    let axis_vert_shader_src = read_file("shaders/axis.vert");
    let axis_frag_shader_src = read_file("shaders/axis.frag");
    glium::Program::from_source(
        display, &axis_vert_shader_src, &axis_frag_shader_src, None).unwrap()
}

pub fn axis_vertices(display: &GlutinFacade) -> glium::VertexBuffer<Vertex> {
    glium::VertexBuffer::new(display, &[
        Vertex { position: [-1000.0,     0.0,     0.0], normal: [0.0, 0.0, 0.0] },
        Vertex { position: [ 1000.0,     0.0,     0.0], normal: [0.0, 0.0, 0.0] },
        Vertex { position: [    0.0, -1000.0,     0.0], normal: [0.0, 0.0, 0.0] },
        Vertex { position: [    0.0,  1000.0,     0.0], normal: [0.0, 0.0, 0.0] },
        Vertex { position: [    0.0,     0.0, -1000.0], normal: [0.0, 0.0, 0.0] },
        Vertex { position: [    0.0,     0.0,  1000.0], normal: [0.0, 0.0, 0.0] },
    ]).unwrap()
}

pub fn model_program(display: &GlutinFacade) -> glium::Program {
    let model_vert_shader_src = read_file("shaders/model.vert");
    let model_frag_shader_src = read_file("shaders/model.frag");
    glium::Program::from_source(
        display, &model_vert_shader_src, &model_frag_shader_src, None).unwrap()
}

pub fn model_buffers(display: &GlutinFacade)
        -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
    let obj = load_obj(get_reader("obj/link.obj")).unwrap();
    (obj.vertex_buffer(display).unwrap(), obj.index_buffer(display).unwrap())
}
