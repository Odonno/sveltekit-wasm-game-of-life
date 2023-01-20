use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use conlife::{Grid, Object};

// Execute animation using a requestAnimationFrame loop.
// https://rustwasm.github.io/docs/wasm-bindgen/examples/request-animation-frame.html

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

fn game() -> web_sys::Element {
    document().get_element_by_id("game").expect("document should have a #game HTML element")
}

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let mut grid = Grid::new(WIDTH, HEIGHT);

    let pentomino = Object::from_string("(1,0) (1,1) (1,2) (0,1) (2,0)").unwrap();
    let block = Object::from_string("(0,0) (0,1) (1,0) (1,1)").unwrap();

    grid.load_object(&pentomino, (9,10)); // Load pentomino at (9,10)
    grid.load_object(&block, (1,1)); // Load block#1 at (1,1)
    grid.load_object(&block, (16,16)); // Load block#2 at (16,16)
    
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let node = document().create_element("div")?;
            let id = format!("{}-{}", x, y);

            node.set_id(&id);
            node.set_class_name("cell");

            if grid.cells[x as usize][y as usize].alive {
                node.set_attribute("style", "background: #000;")?;
            } else {
                node.set_attribute("style", "background: #e5e5e5;")?;
            }

            game().append_child(&node)?;
        }
    }

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

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        grid.advance();

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let id = format!("{}-{}", x, y);
                let node = document().get_element_by_id(&id).unwrap();
    
                if grid.cells[x as usize][y as usize].alive {
                    node.set_attribute("style", "background: #000;").unwrap();
                } else {
                    node.set_attribute("style", "background: #e5e5e5;").unwrap();
                }
            }
        }

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
