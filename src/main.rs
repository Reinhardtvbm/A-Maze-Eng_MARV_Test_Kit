use epr320_dev_test::gui::gui::MARVApp;

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "A-Maze-Eng-MARV Test Kit",
        native_options,
        Box::new(|cc| Box::new(MARVApp::new(cc))),
    );
}
