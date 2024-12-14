use wasm_bindgen::prelude::*;
use web_sys::{KeyboardEvent, window};

mod canvas;
mod direction;
mod snake;

use canvas::Canvas;
use direction::Direction;
use snake::Snake;

use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen(start)]
pub fn start() {
    let canvas = Canvas::new("#canvas", 20, 20);
    let snake = Rc::new(RefCell::new(Snake::new(20, 20)));

    snake.borrow().draw(&canvas);

    let document = window().unwrap().document().unwrap();
    let snake_clone = snake.clone();
    
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        match event.key().as_ref() {
            "ArrowLeft" => snake_clone.borrow_mut().change_direction(Direction::Left),
            "ArrowRight" => snake_clone.borrow_mut().change_direction(Direction::Right),
            "ArrowDown" => snake_clone.borrow_mut().change_direction(Direction::Down),
            "ArrowUp" => snake_clone.borrow_mut().change_direction(Direction::Up),
            _ => {}
        }
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();

    fn game_loop(snake: Rc<RefCell<Snake>>, canvas: Rc<Canvas>) {
        let window = window().unwrap();
        let snake_clone = snake.clone();
        let canvas_clone = canvas.clone();
        
        let closure = Closure::wrap(Box::new(move || {
            snake_clone.borrow_mut().update();
            snake_clone.borrow().draw(&canvas_clone);
            game_loop(snake_clone.clone(), canvas_clone.clone());
        }) as Box<dyn FnMut()>);

        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            100
        ).unwrap();
        closure.forget();
    }

    game_loop(snake, Rc::new(canvas));
}