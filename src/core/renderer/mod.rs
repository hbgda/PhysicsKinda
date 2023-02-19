use sdl2;

pub struct Renderer {
    pub sdl_context: sdl2::Sdl,
    pub canvas: sdl2::render::WindowCanvas
}

impl Renderer {
    pub fn init() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();

        let window = video.window("Tings innit", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let mut canvas = window.into_canvas()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let texture_creator = canvas.texture_creator();

        Renderer {
            sdl_context, canvas
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
