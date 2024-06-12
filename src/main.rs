#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self};
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use std::str;
use winapi;

struct MyApp {
    output: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            output: String::new(),
        }
    }
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
                        // std::process::exit(0);
                    }
                    ui.separator();
                });
            });
        });
        // egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        // ui.add(Label::new(output.clone()));
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.label(output.clone());
                });
        });
        // });
    }
}

fn run_commands() -> String {
    let netstat_output = run_command("netstat", &["-ano"]);
    let lines: Vec<&str> = netstat_output.lines().collect();
    let mut result = String::new();
    for line in lines {
        if line.contains("ESTABLISHED") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let pid = parts.last().unwrap();
            let tasklist_output =
                run_command("tasklist", &["/svc", "/FI", &format!("PID eq {}", pid)]);
            result.push_str(&tasklist_output);
        }
    }
    result
}
fn run_command(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .output()
        .expect("Failed to execute command");
    str::from_utf8(&output.stdout).unwrap().to_string()
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "NetMonGUI",
        native_options,
        Box::new(|_cc| Box::<MyApp>::default()),
    );
}
