//Hilbert curve function based from: https://www.youtube.com/watch?v=dSK-MW-zuAc by Daniel Shiffman
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use nannou::image::{open, RgbaImage};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    counter_start: usize,
    counter_end: usize,
    path: Vec<Point2>,
    egui: Egui,
    settings: Settings,
    scale: f32,
    image: RgbaImage,
    prev_order: u8, 
    image_path: Option<String>,
}
struct Settings {
    r: f32,
    s: f32,
    order: u8,
    show_ui:bool,
    open_file_dialog: bool,
}
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui: Egui = Egui::from_window(&window);
    let settings = Settings {
        r: 1.0,
        s: 150.0,
        order: 6,
        open_file_dialog: false,
        show_ui:true,
    };
    let order = settings.order;
    let n = 2usize.pow(order as u32);
    let total = n * n;
    let window_rect = app.window_rect();
    let len = window_rect.w().min(window_rect.h()) / n as f32;

    let mut path = Vec::with_capacity(total);
    for i in 0..total {
        let mut v = hilbert(i, order);
        v *= len;
        v -= vec2(len * n as f32 / 2.0, len * n as f32 / 2.0);
        path.push(v);
    }
    let image_path = None;
    let image = RgbaImage::new(1, 1);
    let prev_order = settings.order;
    Model {
        path,
        egui,
        settings,
        scale: 1.0,
        counter_start: 0,
        counter_end: 0,
        image,
        prev_order,
        image_path,
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let settings = &mut model.settings;
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut settings.r, 0.1..=40.0).text("r"));
        ui.add(egui::Slider::new(&mut settings.s, 0.1..=10.0).text("s"));
        ui.add(egui::Slider::new(&mut settings.order, 6..=10).text("order"));
        if ui.button("Load Image").clicked() {
            settings.open_file_dialog = true;
        }
    });
    if settings.open_file_dialog {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            model.image_path = Some(path.display().to_string());
            model.image = open(&model.image_path.as_ref().unwrap()).unwrap().to_rgba8();
            settings.open_file_dialog = false;
        }
    }
    if model.counter_start < model.counter_end {
        model.counter_start += model.settings.s as usize;
        model.counter_end -= model.settings.s as usize;
    } else {
        model.counter_start = 0;
        model.counter_end = model.path.len();
    }
    if model.settings.order != model.prev_order {
        model.prev_order = model.settings.order;
        let n = 2usize.pow(model.settings.order as u32);
        let total =  n*n;
        let window_rect = _app.window_rect();
        let len = window_rect.w().min(window_rect.h()) / n as f32;
        let mut path = Vec::with_capacity(total);
        for i in 0..total {
            let mut v = hilbert(i, model.settings.order);
            v *= len;
            v -= vec2(len * n as f32 / 2.0, len * n as f32 / 2.0); 
            path.push(v);
        }
        model.path = path;
        model.counter_start = 0;
        model.counter_end = model.path.len();
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    draw.background().color(BLACK);
    for i in 1..model.counter_start {
        let x = map_range(model.path[i].x, app.window_rect().left(), app.window_rect().right(), 0.0, model.image.width() as f32 * 1.2) as u32;  // Increase width by 20%
        let y = (model.image.height() - 1) - map_range(model.path[i].y, app.window_rect().bottom(), app.window_rect().top(), 0.0, model.image.height() as f32) as u32;
        let pixel_color = model.image.get_pixel(x.min(model.image.width() - 1), y);
        let color = srgba(pixel_color[0] as f32 / 255.0, pixel_color[1] as f32 / 255.0, pixel_color[2] as f32 / 255.0, pixel_color[3] as f32 / 255.0);
        draw.line()
            .start(model.path[i - 1])
            .end(model.path[i])
            .color(color)
            .weight(model.settings.r);
    }
    for i in model.counter_end..model.path.len() {
        let x = map_range(model.path[i].x, app.window_rect().left(), app.window_rect().right(), 0.0, model.image.width() as f32 * 1.2) as u32;  // Increase width by 20%
        let y = (model.image.height() - 1) - map_range(model.path[i].y, app.window_rect().bottom(), app.window_rect().top(), 0.0, model.image.height() as f32) as u32;
        let pixel_color = model.image.get_pixel(x.min(model.image.width() - 1), y);
        let color = srgba(pixel_color[0] as f32 / 255.0, pixel_color[1] as f32 / 255.0, pixel_color[2] as f32 / 255.0, pixel_color[3] as f32 / 255.0);
        draw.line()
            .start(model.path[i - 1])
            .end(model.path[i])
            .color(color)
            .weight(model.settings.r);
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
fn hilbert(i: usize, order: u8) -> Point2 {
    let points = [
        pt2(0.0, 0.0),
        pt2(0.0, 1.0),
        pt2(1.0, 1.0),
        pt2(1.0, 0.0),
    ];
    let mut index = i & 3;
    let mut v = points[index];
    for j in 1..order {
        index = (i >> (2 * j as usize)) & 3;
        let len = 2f32.powi(j as i32);
        v = match index {
            0 => {
                let temp = v.x;
                v.x = v.y;
                v.y = temp;
                v
            }
            1 => {
                v.y += len;
                v
            }
            2 => {
                v.x += len;
                v.y += len;
                v
            }
            3 => {
                let temp = len - 1.0 - v.x;
                v.x = len - 1.0 - v.y;
                v.y = temp;
                v.x += len;
                v
            }
            _ => unreachable!(),
        };
    }
    v
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