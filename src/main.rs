pub mod agent;

use fltk::{app, frame::Frame, prelude::*, window::Window, enums::{Color, self}, draw};

fn main() {
    let app = app::App::default();

    let mut wind = Window::default()
        .with_size(600, 600)
        .center_screen()
        .with_label("Particle Filter Simulation");
    
    wind.set_color(Color::from_rgb(60, 60, 60));

    let mut _frame = Frame::default()
        .with_size(600, 600)
        .center_of(&wind);

    wind.make_resizable(true);
    wind.end();
    wind.show();
    
    let robot = agent::Agent::new(0, 0);
    let _robot_pos = robot.pos.clone();

    // draw loop
    wind.draw(move |_| {
        draw::set_draw_color(Color::White);

        let pos = _robot_pos.borrow();
        draw::draw_rect(pos.x, pos.y, 10, 10);
    });

    // event handler
    wind.handle(move |_, ev| {
        match ev {
            enums::Event::Move => {
                robot.update_pos(app::event_coords().0, app::event_coords().1);
                return true
            }
            _ => return false,
        }
    });

    app::add_idle3(move |_| {
        wind.redraw();
        app::sleep(0.016);
    });

    /* Event handling */
    app.run().unwrap();
}
