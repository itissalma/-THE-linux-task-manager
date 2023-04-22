use serde::Serialize;
use tauri::{Manager, Window};
use sysinfo::{System, SystemExt, CpuExt};
use std::fs::{self, File};
use std::io::Write;
use serde_json::Value;
use std::io::Read;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
/*create tauri command for this code:
fn listprocesses()
{
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    println!("{}","PID   State  PPID  GID  Session   TTY  TTY GID  Flags  Utime  Stime Priority Nice  threads  Start vmemory    CMD   Memory usage  CPU".on_bright_white().bold().red());
    for process in all_processes().unwrap() {
        
        if let Ok(stat) = process.unwrap().stat() {
            let process = psutil::process::Process::new(stat.pid as u32).unwrap();
            let memory_usage = process.memory_info().unwrap().rss();
            let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
            println!("{0: <1}  || {1: <4}|| {2: <4}|| {3: <4}|| {4:<4}|| {5:<4}|| {6:<4}|| {7:<4}|| {8: <4}|| {9: <4}|| {10:<4}|| {11:<4}|| {12:<4} ||{13:<4}|| {14:<4}|| {15:<4} || {16:.2} MB || {17:.2} %", stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0));
        }
    }
    
}*/
#[derive(Serialize)]
struct Process {
    pid: i32,
    state: String,
    ppid: i32,
    pgrp: i32,
    session: i32,
    tty_nr: i32,
    tpgid: i32,
    flags: u32,
    utime: u64,
    stime: u64,
    priority: i32,
    nice: i32,
    num_threads: i32,
    starttime: u64,
    vsize: u64,
    comm: String,
    memory_usage_mb: f64,
    cpu_usage: f64,
}
#[derive(Serialize)]
struct ProcessList {
    processes: Vec<Process>,
}
#[tauri::command]
fn listprocesses() -> String {
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    let mut processes: Vec<Process> = Vec::new();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            let process = psutil::process::Process::new(stat.pid as u32).unwrap();
            let memory_usage = process.memory_info().unwrap().rss();
            let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
            let cpu_usage = (((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0);
            let process = Process {
                pid: stat.pid,
                state: stat.state,
                ppid: stat.ppid,
                pgrp: stat.pgrp,
                session: stat.session,
                tty_nr: stat.tty_nr,
                tpgid: stat.tpgid,
                flags: stat.flags,
                utime: stat.utime,
                stime: stat.stime,
                priority: stat.priority,
                nice: stat.nice,
                num_threads: stat.num_threads,
                starttime: stat.starttime,
                vsize: stat.vsize,
                comm: stat.comm,
                memory_usage_mb: memory_usage_mb,
                cpu_usage: cpu_usage,
            };
            processes.push(process);
        }
    }
    let processlist = ProcessList {
        processes: processes,
    };
    let json = serde_json::to_string(&processlist).unwrap();
    println!("{}", json);
    json
}
//create a tauri command get_system_info that returns a JSON string
#[tauri::command]
fn get_system_info() -> String {
  let system = System::new();
  let system_name = system.name().unwrap_or_default();
  let kernel_version = system.kernel_version().unwrap_or_default();
  let os_version = system.os_version().unwrap_or_default();
  let host_name = system.host_name().unwrap_or_default();
  let json = format!(
      r#"{{"hostname":"{}","system_name":"{}","kernel_version":"{}","os_version":"{}"}}"#,
      host_name, system_name, kernel_version, os_version
  );
 println!("{}", json);
  json
}

#[tauri::command]
fn get_system_usage() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let total_mem = sys.total_memory();
    let free_mem = sys.free_memory();
    let used_mem = total_mem - free_mem;
    let mem_percent = (used_mem as f32 / total_mem as f32) * 100.0;
    let total_swap = sys.total_swap();
    let free_swap = sys.free_swap();
    let used_swap = total_swap - free_swap;
    let swap_percent = used_swap as f32 / total_swap as f32 * 100.0;
    let mut cpu_use = 0.0;
    for cpu in sys.cpus() {
        cpu_use += cpu.cpu_usage();
    }
    let num_cpus = sys.cpus().len() as f32;
    let cpu_percent = cpu_use / num_cpus;
    let json = format!(
        r#"{{"mem_percent":{:.1},"swap_percent":{:.1},"cpu_percent":{:.1}}}"#,
        mem_percent, swap_percent, cpu_percent
    );
    println!("{}", json);
    json
}

#[tauri::command]
fn log_to_terminal(message: String) {
  println!("{}", message);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_system_info, get_system_usage, log_to_terminal])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
