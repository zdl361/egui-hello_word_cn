#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 340.0)),
        ..Default::default()
    };
    eframe::run_native(
        "手写器",
        options,
        Box::new(|_cc| Box::<MyApp>::new(MyApp::new(_cc))),
    )
}

struct MyApp {
    name: String,
    age: u32,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}
impl MyApp{
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		//load_harmony_os_font(& cc.egui_ctx);
        Self::default()
    }
	
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

pub fn load_harmony_os_font(ctx: &egui::Context){
    let mut fonts = eframe::egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert("HarmonyOS_Sans".to_owned(),
                           eframe::egui::FontData::from_static(include_bytes!("../resources/HarmonyOS_Sans_Regular.ttf"))); // .ttf and .otf supported
    fonts.font_data.insert("HarmonyOS_Sans_SC".to_owned(),
                           eframe::egui::FontData::from_static(include_bytes!("../resources/HarmonyOS_Sans_SC_Regular.ttf"))); 
	fonts.font_data.insert("HarmonyOS_Sans_TC".to_owned(),
                           eframe::egui::FontData::from_static(include_bytes!("../resources/HarmonyOS_Sans_TC_Regular.ttf"))); 
    // Put my font first (highest priority):
    fonts.families.get_mut(&eframe::egui::FontFamily::Proportional).unwrap()
        .insert(0, "HarmonyOS_Sans_TC".to_owned());
	fonts.families.get_mut(&eframe::egui::FontFamily::Proportional).unwrap()
        .insert(0, "HarmonyOS_Sans_SC".to_owned());
    fonts.families.get_mut(&eframe::egui::FontFamily::Proportional).unwrap()
        .insert(0, "HarmonyOS_Sans".to_owned());

    // Put my font as last fallback for monospace:


    // let mut ctx = egui::CtxRef::default();
    ctx.set_fonts(fonts);
}