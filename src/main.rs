use fltk::{app, button::Button, input::Input, prelude::*, window::Window};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 300, 400, Some("Калькулятор"));

    let mut display = Input::new(10, 10, 280, 40, None);
    display.set_readonly(true);

    let operations = Rc::new(RefCell::new(String::new()));

    let mut buttons = vec![];
    let button_labels = vec![
        "7", "8", "9", "/",
        "4", "5", "6", "*",
        "1", "2", "3", "-",
        "0", ".", "=", "+"
    ];

    for (i, label) in button_labels.iter().enumerate() {
        let label = label.to_string();  // Превращаем строку в `String`
        let x = (10 + (i % 4) * 70) as i32;
        let y = (60 + (i / 4) * 70) as i32;
        let mut btn = Button::new(x, y, 60, 60, Some(label.as_str()));  // Преобразуем `String` в `&str`

        btn.set_callback({
            let label = label.clone();
            let operations = operations.clone();
            let mut display = display.clone();
            move |_| {
                let mut ops = operations.borrow_mut();
                match label.as_str() {
                    "=" => {
                        if let Ok(result) = eval(&ops) {
                            display.set_value(&result.to_string());
                        } else {
                            display.set_value("Ошибка");
                        }
                        ops.clear();
                    }
                    _ => {
                        ops.push_str(label.as_str());
                        display.set_value(&ops);
                    }
                }
            }
        });

        buttons.push(btn);
    }



    wind.end();
    wind.show();

    app.run().unwrap();
}

fn eval(expression: &str) -> Result<f64, &str> {
    let expr = meval::eval_str(expression);
    match expr {
        Ok(result) => Ok(result),
        Err(_) => Err("Ошибка"),
    }
}
