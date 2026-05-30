#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use relationship_calculator::RelationshipCalculatorApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("关系计算器")
            .with_inner_size([1100.0, 760.0])
            .with_min_inner_size([980.0, 680.0]),
        ..Default::default()
    };

    eframe::run_native(
        "关系计算器",
        options,
        Box::new(|cc| Ok(Box::new(RelationshipCalculatorApp::new(cc)))),
    )
}
