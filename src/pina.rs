use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use nannou::noise::{NoiseFn, Perlin};
#[derive(Clone, Copy, Debug, PartialEq)]
enum AnimationMode {
    Default,
    Reverse,
    Perlin,
}
fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    egui: Egui,
    settings: Settings,
    perlin: Perlin,
    scale: f32,

}

struct Settings {
    animation: bool,
    animation_speed: f32,
    a:f32,
    animation_mode: AnimationMode,
    n:usize,
    num_spirals: usize,
    show_ui:bool,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let perlin = Perlin::new();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings {
        animation: false,
        animation_speed: 0.01,
        animation_mode: AnimationMode::Default,
        a: 20.0,
        n: 360,
        num_spirals: 4,
        show_ui:true,
    };
    Model { egui, settings, perlin,scale: 1.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let ctx = egui.begin_frame();
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.checkbox(&mut model.settings.animation, "Animate");
        ui.vertical(|ui| {
            ui.label("Speed");
            ui.add(egui::Slider::new(
                &mut model.settings.animation_speed,
                0.1..=10.1,

            ));
            ui.label("n");
            ui.add(egui::Slider::new(
                &mut model.settings.n,
                1..=360,
            ));
            ui.label("num_spirals");
            ui.add(egui::Slider::new(
                &mut model.settings.num_spirals,
                1..=10,
            ));

            ui.radio_value(&mut model.settings.animation_mode, AnimationMode::Default, "Default");
            ui.radio_value(&mut model.settings.animation_mode, AnimationMode::Reverse, "Reverse");
            ui.radio_value(&mut model.settings.animation_mode, AnimationMode::Perlin, "Perlin");
            ui.label("a:");
            ui.add(egui::Slider::new(
                &mut model.settings.a,
                0.1..=100.0,
            ));

        });
    });
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale); 
    let win = app.window_rect();
    let perlin = &model.perlin;

    draw.background().color(rgba(0.5, 0.5, 0.5, 1.0));
    let base_intervals = [20.0, 11.25, 8.18, 6.206,4.102, 4.12,7.05,5.18,3.13];
    let intervals: Vec<f32> = base_intervals.iter().take(model.settings.num_spirals).copied().collect();

    let rect_size = model.settings.a; 


    let mut white = true;
    for (j, interval) in intervals.iter().enumerate() {
        white = !white;
        for i in (0..model.settings.n).step_by(*interval as usize) {
            let color = if white { WHITE } else { BLACK };
            white = !white;
            let i = i as f32;
            let x = win.w() / 50.0 + i.to_radians().cos() * (125.0 + (j as f32 * 80.0));
            let y = win.h() / 50.0 + i.to_radians().sin() * (125.0 + (j as f32 * 80.0));
            let base_rotation = if j % 2 == 0 { -15.0 } else { 15.0 };
            let rotation = i.to_radians() + base_rotation.to_radians();

            let rotation = if model.settings.animation {
                match model.settings.animation_mode {
                    AnimationMode::Default => rotation + model.settings.animation_speed * app.time,
                    AnimationMode::Reverse => {
                        let direction = if white { -1.0 } else { 1.0 };
                        rotation + model.settings.animation_speed * app.time * direction
                    }
                    AnimationMode::Perlin => {
                        let perlin_noise_value = perlin.get([x as f64, y as f64]) as f32;
                        rotation + model.settings.animation_speed * app.time * perlin_noise_value
                    }
                }
            } else {
                rotation
            };


            draw.rect()
            .x_y(x, y)
            .w_h(rect_size, rect_size)
            .rotate(rotation)
            .stroke_weight(4.0)
            .stroke_color(color)
            .color(color)
            .no_fill();
    }
    }

    draw.to_frame(app, &frame).unwrap();
    if model.settings.show_ui {
        model.egui.draw_to_frame(&frame).unwrap();
    }  
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
          .project_path()
          .expect("failed to locate project directory")
          .join("frames") 
          .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path); 

    } 
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::MouseWheel { delta, .. } = event {
        let cursor_over_egui = model.egui.ctx().wants_pointer_input();
        if !cursor_over_egui {
            match delta {
                nannou::winit::event::MouseScrollDelta::LineDelta(_, y) => {
                    model.scale *= 1.0 + *y * 0.05;
                    model.scale = model.scale.max(0.1).min(10.0);
                }
                _ => (),
            }
        }
    }
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = _app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
}