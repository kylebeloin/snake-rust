use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

fn get_element_by_id(id: &str) -> web_sys::HtmlElement {
    document()
        .get_element_by_id(id)
        .expect("should have an element with id")
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .expect("element with id should be a HtmlElement")
}

fn start_button() -> web_sys::HtmlElement {
    get_element_by_id("start")
}

fn stop_button() -> web_sys::HtmlElement {
    get_element_by_id("stop")
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spawn_index)]
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World { width: 16, snake: Snake::new(10) }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
       self.snake.body[0].0
    }
}
#[wasm_bindgen]
pub struct Canvas {
    canvas: web_sys::HtmlCanvasElement,
    context: Rc<web_sys::CanvasRenderingContext2d>,
}

impl Canvas {
    pub fn new() -> Canvas {
        let container = get_element_by_id("game");
        let canvas = document()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        container.append_child(&canvas).unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        
        let context = Rc::new(context);
        Canvas { canvas, context }
    }
// Get canvas as Result
    pub fn get_canvas(&self) -> Result<web_sys::HtmlCanvasElement, JsValue> {
        Ok(self.canvas.clone())
    }

    pub fn get_context(&self) -> Rc<web_sys::CanvasRenderingContext2d> {
        self.context.clone()
    }

    pub fn draw(&self, world: &World) {
        let canvas = self.get_canvas().unwrap();
        let context = self.get_context();
        let cell_size = 20;
        let width = world.width() as u32 * cell_size;
        let head_idx = world.snake_head_idx();
        let x = (head_idx % world.width()) as u32 * cell_size;
        let y = (head_idx / world.width()) as u32 * cell_size;

        canvas.set_width(width);
        canvas.set_height(width);
        context.set_fill_style(&JsValue::from_str("black"));
        context.fill_rect(0.0, 0.0, width as f64, width as f64);
        context.set_fill_style(&JsValue::from_str("green"));
        context.fill_rect(x as f64, y as f64, cell_size as f64, cell_size as f64);
    }
}
#[wasm_bindgen]
struct Controls {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}
#[wasm_bindgen]
pub struct UI {
    controls: Rc<RefCell<Controls>>,
    mouse_down: Rc<RefCell<bool>>,
   
}
#[wasm_bindgen]
impl UI {
    pub fn new() -> UI {
        let controls = Rc::new(RefCell::new(Controls {
            left: false,
            right: false,
            up: false,
            down: false,
        }));

        let mouse_down = Rc::new(RefCell::new(false));
      

        UI { controls, mouse_down  }
    }

    pub fn update(&self, world: &mut World) {
        let controls = self.controls.borrow();
        
        if controls.left {
            world.snake.body[0].0 -= 1;
        }
        if controls.right {
            world.snake.body[0].0 += 1;
        }
        if controls.up {
            world.snake.body[0].0 -= world.width;
        }
        if controls.down {
            world.snake.body[0].0 += world.width;
        }
    }

    pub fn init(&self) -> Result<(), JsValue> {
        let window = window();
        
        let controls = self.controls.clone();
        let keydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let mut controls = controls.borrow_mut();
            match event.key().as_ref() {
                "ArrowLeft" => controls.left = true,
                "ArrowRight" => controls.right = true,
                "ArrowUp" => controls.up = true,
                "ArrowDown" => controls.down = true,
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        let controls = self.controls.clone();
        let keyup = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let mut controls = controls.borrow_mut();
            match event.key().as_ref() {
                "ArrowLeft" => controls.left = false,
                "ArrowRight" => controls.right = false,
                "ArrowUp" => controls.up = false,
                "ArrowDown" => controls.down = false,
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        let mouse_down = self.mouse_down.clone();
        let start = Closure::wrap(Box::new(move | _event: web_sys::MouseEvent| {
            let mut mouse_down = mouse_down.borrow_mut();
            *mouse_down = true;
            console::log_1(&JsValue::from_str("start"));
        }) as Box<dyn FnMut(_)>);

        
        let start_button = start_button();
        start_button.add_event_listener_with_callback("mousedown", start.as_ref().unchecked_ref())?;

        let mouse_down = self.mouse_down.clone();
        let end = Closure::wrap(Box::new(move | _event: web_sys::MouseEvent| {
            let mut mouse_down = mouse_down.borrow_mut();
            *mouse_down = false;
            console::log_1(&JsValue::from_str("end"));
        }) as Box<dyn FnMut(_)>);

        let end_button = stop_button();
        end_button.add_event_listener_with_callback("mouseup", end.as_ref().unchecked_ref())?;

        window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())?;
        window.add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())?;
        
        keydown.forget();
        keyup.forget();
        start.forget();
        end.forget();
        Ok(())
    }
}


#[wasm_bindgen]
struct Game {
    started: bool,
    canvas: Canvas,
    world: World,
    ui: UI,
}
#[wasm_bindgen]
impl Game {
    fn new() -> Game {
       
        Game { started: false, canvas: Canvas::new(), world: World::new(), ui: UI::new() }
    }

    fn init(&mut self) -> Result<(), JsValue> {
        self.ui.init().unwrap(); 
        self.start();
        Ok(())
    }
        
    fn start(&mut self) {
        self.started = true;
        // self.ui.init().unwrap();
    }

    fn stop(&mut self) {
        self.started = false;
    }

    fn update(&mut self) {
        if self.started {
            self.ui.update(&mut self.world);
           
            self.canvas.draw(&self.world);
        }
    }
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
    // number of times. After it's done we want all our resources cleaned up. To
    // achieve this we're using an `Rc`. The `Rc` will eventually store the
    // closure we want to execute on each frame, but to start out it contains
    // `None`.
    //
    // After the `Rc` is made we'll actually create the closure, and the closure
    // will reference one of the `Rc` instances. The other `Rc` reference is
    // used to store the closure, request the first frame, and then is dropped
    // by this function.
    //
    // Inside the closure we've got a persistent `Rc` reference, which we use
    // for all future iterations of the loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    let mut game = Game::new();
    game.init().unwrap();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
       
   
        if i > 3000 {

            body().set_text_content(Some("All done!"));
            game.stop();
            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        game.update();
        if game.started {
            i += 1;
            let container = get_element_by_id("text");
            let text = format!("requestAnimationFrame has been called {} times.", i);
            container.set_text_content(Some(&text));
        }

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        

        // Schedule ourself for another requestAnimationFrame callback, and call canvas draw.
        request_animation_frame(f.borrow().as_ref().unwrap());

    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}