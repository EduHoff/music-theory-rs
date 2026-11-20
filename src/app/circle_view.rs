use crate::app::AppWindow;
use slint::ComponentHandle;
use std::cell::RefCell;
use std::rc::Rc;

pub fn setup(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    let last_angle: Rc<RefCell<Option<f32>>> = Rc::new(RefCell::new(None));

    ui.on_circle_drag_event(move |mx, my, cx, cy, is_pressed| {
        let Some(ui) = ui_handle.upgrade() else {
            return;
        };

        if !is_pressed {
            *last_angle.borrow_mut() = None;
            return;
        }

        let dx = mx - cx;
        let dy = my - cy;
        let current_mouse_angle = dy.atan2(dx);

        let mut last_ang_ref = last_angle.borrow_mut();

        if let Some(last_ang) = *last_ang_ref {
            let mut delta = current_mouse_angle - last_ang;

            if delta > std::f32::consts::PI {
                delta -= 2.0 * std::f32::consts::PI;
            } else if delta < -std::f32::consts::PI {
                delta += 2.0 * std::f32::consts::PI;
            }

            let delta_deg = delta * (180.0 / std::f32::consts::PI);

            let new_rotation = ui.get_circle_rotation() + delta_deg;
            ui.set_circle_rotation(new_rotation);
        }

        *last_ang_ref = Some(current_mouse_angle);
    });
}
