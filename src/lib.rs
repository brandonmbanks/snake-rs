mod random;
mod snake;

use js_sys::Function;
use random::get_random_number;
use snake::{Direction, Position, SnakeGame};
use std::cell::RefCell;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, HtmlDivElement, HtmlElement, KeyboardEvent};

thread_local! {
    static GAME: RefCell<SnakeGame> = RefCell::new(SnakeGame::new(20, 20));

    static HANDLE_GAME_LOOP: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
        GAME.with(|game| game.borrow_mut().game_loop());
        render();
      }) as Box<dyn FnMut()>);

    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> =
      Closure::wrap(Box::new(|evt: KeyboardEvent| GAME.with(|game| {
        let direction = match &evt.key()[..] {
          "ArrowUp" => Some(Direction::Up),
          "ArrowRight" => Some(Direction::Right),
          "ArrowDown" => Some(Direction::Down),
          "ArrowLeft" => Some(Direction::Left),
          _ => None,
        };

        if let Some(direction) = direction {
          game.borrow_mut().change_direction(direction);
        }
      })) as Box<dyn FnMut(KeyboardEvent)>)
}

#[wasm_bindgen(start)]
pub fn main() {
    HANDLE_GAME_LOOP.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                200,
            )
            .unwrap_throw()
    });

    HANDLE_KEYDOWN.with(|handle_keydown| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback(
                "keydown",
                handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });

    render();
}

pub fn render() {
    GAME.with(|game| {
        let game = game.borrow();

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let root_container = document
            .get_element_by_id("root")
            .unwrap_throw()
            .dyn_into::<HtmlElement>()
            .unwrap_throw();

        root_container.set_inner_html("");

        let width = game.width;
        let height = game.height;

        root_container
            .style()
            .set_property("display", "inline-grid")
            .unwrap_throw();

        root_container
            .style()
            .set_property(
                "grid-template",
                &format!("repeat({}, auto) / repeat({}, auto)", height, width),
            )
            .unwrap_throw();

        for y in 0..height {
            for x in 0..width {
                let pos = Position { x, y };

                let field_element = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap_throw();

                field_element.set_class_name("field");
                field_element.set_inner_text({
                    if pos == game.food {
                        "🍎"
                    } else if game.snake[0] == pos {
                        "❇️"
                    } else if game.snake.contains(&pos) {
                        "🟩"
                    } else {
                        " "
                    }
                });

                root_container.append_child(&field_element).unwrap_throw();
            }
        }

        if game.finished {
            let state_container = document
                .get_element_by_id("state")
                .unwrap_throw()
                .dyn_into::<HtmlElement>()
                .unwrap_throw();

            state_container.set_inner_html("<p>Game Over</p>");
        }
    })
}
