use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    egui: Egui,
    settings:Settings,
    path: Vec<(Point2, usize)>,
    counter: usize,
    dx: isize,
    dy: isize,
    n: isize,
    x: isize,
    y: isize,
    numbers: Vec<bool>,
    scale:f32,
}
struct Settings {
    restart:bool,
    stop:bool,
    show_ui:bool,
}

fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .size(800, 800)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let size: usize = 10000;
    let window = app.window(_window_id).unwrap();

    let egui: Egui = Egui::from_window(&window);

    let mut numbers = vec![false; size];
    for i in 2..size {
        if is_prime(i) {
            numbers[i] = true;
        }
    }
    let settings = Settings {
        restart:false,
        stop:false,
        show_ui:true,
    };
    Model {
        path: Vec::new(),
        counter: 1,
        dx: 1,
        dy: 0,
        n: 1,
        x: 0,
        y: 0,
        numbers,
        egui,
        settings,
        scale:1.0,
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let settings = &mut model.settings;
    
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        if ui.button("Restart").clicked() {
            settings.restart = true;
        }
        if ui.button("Stop/Resume").clicked() {
            settings.stop = !settings.stop;
        }
    });
    if settings.stop {
        return;
    }

    if settings.restart {
        model.counter = 1;
        model.n = 1;
        model.dx = 7;
        model.dy = 0;
        model.x = 0;
        model.y = 0;
        model.path.clear();
        settings.restart = false;
    }
    let i = model.counter;
    let x = model.x;
    let y = model.y;
    let pt = pt2(x as f32, y as f32);
    model.path.push((pt, i));
    if model.n * model.n + 1 == i as isize {
        model.dy = ((model.n % 2) * 2 - 1) * 7;
        model.dx = 0;
        model.n += 1;
    } else if model.n * model.n - model.n + 1 == i as isize {
        model.dx = ((model.n % 2) * 2 - 1) * 7;
        model.dy = 0;
    }
    model.x += model.dx;
    model.y += model.dy;
    model.counter += 1;
    if model.counter >= model.numbers.len() {
        model.counter = 1;
        model.n = 1;
        model.dx = 7;
        model.dy = 0;
        model.x = 0;
        model.y = 0;
        model.path.clear();
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    draw.background().color(BLACK);
    for &(point, num) in &model.path {
        if model.numbers[num] {
            draw.ellipse()
                .xy(point)
                .radius(3.0)
                .color(WHITE);
        } else {
            draw.ellipse()
                .xy(point)
                .radius(1.5)
                .color(RED);
        }
    }
    let primes: Vec<String> = model.path
        .iter()
        .filter(|&(_, num)| model.numbers[*num])
        .map(|&(_, num)| num.to_string())
        .collect();
    let prime_text = primes.join(", ");
    draw.text(&prime_text)
        .color(WHITE)
        .xy(vec2(-350.0, 350.0))
        .font_size(15);
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
fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..((n as f64).sqrt() as usize + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
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
