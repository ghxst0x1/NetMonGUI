#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, Align, Context, Layout, SidePanel, CentralPanel, ScrollArea};
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use std::str;
use winapi::um::winbase::CREATE_NO_WINDOW;

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
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let Self { output } = self;
        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Commands");

            if ui.button("START").clicked() {
                match run_commands() {
                    Ok(result) => *output = result,
                    Err(e) => *output = format!("Error: {}", e),
                }
            }

            ui.vertical(|ui| {
                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    if ui.button("QUIT").clicked() {
                        std::process::exit(0);
                    }
                    ui.separator();
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                ui.label(output.clone());
            });
        });
    }
}

fn run_commands() -> Result<String, Box<dyn std::error::Error>> {
    let netstat_output = run_command("netstat", &["-ano"])?;
    let lines: Vec<&str> = netstat_output.lines().collect();
    let mut result = String::new();
    for line in lines {
        if line.contains("ESTABLISHED") {
            if let Some(pid) = line.split_whitespace().last() {
                let tasklist_output = run_command("tasklist", &["/svc", "/FI", &format!("PID eq {}", pid)])?;
                result.push_str(&tasklist_output);
            }
        }
    }
    Ok(result)
}

fn run_command(command: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "NetMonGUI",
        native_options,
        Box::new(|_cc| Box::<MyApp>::default()),
    ).expect("Failed to start eframe");
}