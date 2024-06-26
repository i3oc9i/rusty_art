use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    circle_points: Vec<Vec<Point2>>,
    settings: Settings,
    egui: Egui,
}

struct Settings {
    num_circles: usize,
    const_: f32,
    frequency: f32,
    amplitude: f32,
    phase: f32,
    num_points: usize,
    radius: f32,
    r: f32,
    x: f32,
    y: f32,
    z: f32,
    t:f32,
    c: usize,
    ani: bool,
    p:f32,
    show_ui:bool,

}
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings {
        num_circles: 40,
        const_: 25.0,
        frequency: 125.0,
        amplitude: 54.4,
        phase: 1.0,
        num_points: 155,
        radius: 1.5,
        r:4.0,
        x: 2.0,
        y: 2.0,
        z: 2.0,
        t:0.5,
        c: 0,
        ani: true,
        p: 1.0,
        show_ui:true,


    };
    let circle_points = generate_circle_points(&settings, &window.rect());

    Model {
        circle_points,
        settings,
        egui,
        
    }
}

fn generate_circle_points(settings: &Settings, window_rect: &Rect) -> Vec<Vec<Point2>> {
    let center = window_rect.xy();
    let radius = window_rect.w().min(window_rect.h()) / settings.z;
    let circle_radius = radius / (settings.num_circles as f32);
    let mut circle_points = Vec::with_capacity(settings.num_circles);

    for i in 0..settings.num_circles {
        let mut points = Vec::with_capacity(settings.num_points);
        for j in 0..settings.num_points {
            let angle = j as f32 * settings.r * settings.p*PI / (settings.num_points as f32);
            let x = center.x + (angle.sin() * circle_radius * (i as f32 + settings.x)) * (1.0);
            let y = center.y + (angle.cos() * circle_radius * (i as f32 + settings.y)) * (1.0);
            points.push(pt2(x, y));
        }
        circle_points.push(points);
    }

    circle_points
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    if app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Num Circles:");
        ui.add(egui::Slider::new(&mut settings.num_circles, 1..=255));
        ui.label("Const:");
        ui.add(egui::Slider::new(&mut settings.const_, 1.0..=255.0));
        ui.label("Frequency:");
        ui.add(egui::Slider::new(&mut settings.frequency, 1.0..=255.0));
        ui.label("Amplitude:");
        ui.add(egui::Slider::new(&mut settings.amplitude, 1.0..=255.0));
        ui.label("Phase:");
        ui.label("Num Points:");
        ui.add(egui::Slider::new(&mut settings.num_points, 1..=500));
        ui.label("Radius:");
        ui.add(egui::Slider::new(&mut settings.radius, 0.1..=10.0));
        ui.label("R:");
        ui.add(egui::Slider::new(&mut settings.r, 0.1..=10.0));
        ui.label("X:");
        ui.add(egui::Slider::new(&mut settings.x, 0.1..=10.0));
        ui.label("Y:");
        ui.add(egui::Slider::new(&mut settings.y, 0.1..=10.0));
        ui.label("Z:");
        ui.add(egui::Slider::new(&mut settings.z, 0.1..=10.0));
        ui.label("T:");
        ui.add(egui::Slider::new(&mut settings.t, 0.1..=10.0));
        ui.label("P:");
        ui.add(egui::Slider::new(&mut settings.p, 0.1..=10.0));
        ui.label(format!("Current Color Pattern: {}", settings.c));
        if ui.button("Switch Color Pattern").clicked() {
            settings.c = (settings.c + 1) % 14;
        }
        if ui.button("Toggle Background Animation").clicked() {
            settings.ani = !settings.ani;
        }
    });
    model.circle_points = generate_circle_points(&model.settings, &app.window_rect());

    for i in 0..model.settings.num_circles {
        for j in 0..model.settings.num_points {
            let x = model.circle_points[i][j].x
                + (-model.settings.const_ * PI * model.settings.frequency * j as f32 / model.settings.num_points as f32 + model.settings.phase)
                    .sin()
                    * model.settings.amplitude;
            let y = model.circle_points[i][j].y
                + (model.settings.const_ * PI * model.settings.frequency * j as f32 / model.settings.num_points as f32 + model.settings.phase)
                    .cos()
                    * model.settings.amplitude;
            model.circle_points[i][j] = pt2(x, y);

        }
    }
    model.settings.phase += 0.01;
}
            
fn raw_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
}


fn view(app: &App, model: &Model, frame: Frame) {
let settings = &model.settings;
let draw = app.draw();
if settings.ani {
    draw.background().color(hsla(app.time.sin() / 2.0, 0.5, 0.5, 1.0));
} else {
    draw.background().color(BLACK);
}           
for i in 0..settings.num_circles {
    let progress = i as f32 / settings.num_circles as f32;
    let color = match settings.c {
        0 => {
            let hue: f32 = settings.t * ((progress * 2.0 * PI) + app.time).sin() + 0.5;
            hsla(hue, 0.6, 0.5, 1.0)
        }
        1 => {
            let t = app.time;
            let hue = progress + settings.t  * (t + i as f32 * 0.1).cos();
            let saturation = progress +settings.t  * (t + i as f32 * 0.2).cos();
            let lightness = progress + settings.t  * (t + i as f32 * 0.3).cos();
            hsla(hue, saturation, lightness, 1.0)

        }
        2 => {
            let hue: f32 = settings.t * ((progress * 2.0 * PI) + app.time).sin() + 0.5;
            hsla(hue, 1.0, 0.5, 1.0)
        }

        3 => {
            let hue: f32 = 1.5 * app.time.cos() * progress;
            let saturation = 1.5 + 1.5 * settings.t;
            let lightness = 1.5 + 0.5 * progress;
            hsla(hue, saturation, lightness, 1.0)
        }
        4 => {
            let hue: f32 = 0.5 + 0.5 * (settings.t+app.time + progress * PI).sin();
            let saturation = progress;
            let lightness = 0.4 + 0.4 * (settings.t+app.time + progress * PI).cos();
            hsla(hue, saturation, lightness, 1.0)
        }
        5 => {
            let hue = progress;
            let saturation = 1.0 - progress;
            let lightness = 0.4 + 0.4 * (settings.t+app.time + progress * PI).sin();
            hsla(hue, saturation, lightness, 1.0)
        }
        6 => {
            let t = app.time;
            let hue = progress + settings.t * (t + i as f32 * 0.15).sin();
            let saturation = 0.5 + settings.t * 0.5 * (t + i as f32 * 0.25).cos();
            let lightness = 0.5 + settings.t * 0.25 * (t + i as f32 * 0.1).sin() + settings.t * 0.25 * (t + i as f32 * 0.2).cos();
            hsla(hue, saturation, lightness, 1.0)
        }

        7 => {
            let t = app.time;
            let hue = progress + settings.t * (t + i as f32 * 0.12).cos();
            let saturation = 0.5 + settings.t * 0.3 * (t + i as f32 * 0.2).sin();
            let lightness = 0.4 + settings.t * 0.4 * (t + i as f32 * 0.25).cos() + settings.t * 0.2 * (t + i as f32 * 0.15).sin();
            hsla(hue, saturation, lightness, 1.0)
        }

        8 => {
            let t = app.time;
            let hue = progress + settings.t * (t + i as f32 * 0.22).sin();
            let saturation = 1.5 + settings.t * 1.5 * (t + i as f32 * 0.18).cos();
            let lightness = 0.5 + settings.t * 0.35 * (t + i as f32 * 0.3).sin() + settings.t * 0.15 * (t + i as f32 * 0.28).cos();
            hsla(hue, saturation, lightness, 1.0)
        }
        9 => {
            let golden_ratio: f32 = (1.0 + 5.0f32.sqrt()).sin() / 2.0;
            let t = app.time.sin();
            let hue = progress.sin() * 1.0 + settings.t * 30.0;
            let saturation = 0.5 + 0.5 * ((golden_ratio * t).sin() * 0.5 + 0.5);
            let lightness = 0.5 + 0.5 * ((golden_ratio * t * 2.0).sin() * 0.5 + 0.5);
            hsla(hue, saturation, lightness, 1.0)
        }
        10 => {
            let hue = progress;
            let saturation = 1.0 - progress;
            let lightness = 0.5 + 0.5 * (settings.t + app.time + progress * PI).sin();
            let adjusted_lightness = if lightness > 0.8 { 0.3 } else { lightness };
            hsla(hue, saturation, adjusted_lightness, 1.0)
        }
        11 => {
            let hue = progress;
            let saturation = settings.t - progress;
            let lightness = settings.z + 0.5 * progress;
            let adjusted_lightness = if lightness > settings.y { 0.3 } else { lightness };
            hsla(hue, saturation, adjusted_lightness, 1.0)
        }
        12 => {
            let saturation = 1.0 - progress;
            let lightness = 0.5 + 0.5 * (settings.t + app.time + progress * PI).sin();
            hsla(1.0, saturation, lightness, 1.0)
        }
        13 => {
            hsla(1.0, 1.0, 1.0, 1.0)
        }
        _ => unreachable!(),

    };
    draw.polyline()
        .weight(settings.radius)
        .points(model.circle_points[i].clone())
        .color(color);
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