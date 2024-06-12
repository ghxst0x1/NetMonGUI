#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::os::windows::process::CommandExt;

#[derive(Default)]
struct MyApp {
    output: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { output } = self;
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Commands");

            if ui.button("START").clicked() {
                *output = run_commands();
            }
            ui.vertical(|ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui.button("QUIT").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    ui.separator();
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.label(output.clone());
                });
        });
    }
}

fn run_commands() -> String {
    let netstat_output = run_command("netstat", &["-ano"]);
    let result: Vec<String> = netstat_output
        .lines()
        .filter(|line| line.contains("ESTABLISHED"))
        .flat_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let pid = parts.last().unwrap();
            run_command("tasklist", &["/svc", "/FI", &format!("PID eq {}", pid)])
                .lines()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .collect();
    result.join("\n")
}

fn run_command(command: &str, args: &[&str]) -> String {
    use std::process::{Command, Stdio};

    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .output()
        .map(|output| output.stdout)
        .map_err(|err| err.to_string())
        .unwrap_or_default();
    String::from_utf8_lossy(&output).to_string()
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "NetMonGUI",
        native_options,
        Box::new(|_cc| Box::<MyApp>::default()),
    );
}
