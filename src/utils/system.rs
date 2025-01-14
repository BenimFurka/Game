use std::sync::mpsc;
use macroquad::time::get_time;
use sysinfo::{System, SystemExt, ProcessExt, CpuExt, Pid};

#[derive(Debug)]
pub enum SystemError {
    ProcessNotFound,
    MemoryReadError,
    CpuReadError,
}

pub struct SystemInfo {
    pub process_memory: u64,
    pub cpu_usage: f32,
    pub fps: u32,
    frames_count: u32,
    last_fps_update: f64,
    last_update: f64,
    receiver: mpsc::Receiver<Result<(u64, f32), SystemError>>,
}

impl SystemInfo {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let pid = std::process::id() as usize;

        std::thread::spawn(move || {
            let mut sys = System::new_all();
            loop {
                sys.refresh_all();
                let result = sys.process(Pid::from(pid))
                    .ok_or(SystemError::ProcessNotFound)
                    .and_then(|process| {
                        let memory = process.memory().checked_div(1024 * 1024)
                            .ok_or(SystemError::MemoryReadError)?;
                        let cpu = process.cpu_usage();
                        if cpu < 0.0 {
                            return Err(SystemError::CpuReadError);
                        }
                        Ok((memory, cpu))
                    });
                
                let _ = sender.send(result);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });

        Self {
            process_memory: 0,
            cpu_usage: 0.0,
            fps: 0,
            frames_count: 0,
            last_fps_update: get_time(),
            last_update: 0.0,
            receiver,
        }
    }

    pub fn update(&mut self) {
        self.frames_count += 1;
        let current_time = get_time();
        if current_time - self.last_fps_update >= 1.0 {
            self.fps = self.frames_count;
            self.frames_count = 0;
            self.last_fps_update = current_time;
        }
        if let Ok(result) = self.receiver.try_recv() {
            match result {
                Ok((memory, cpu)) => {
                    self.process_memory = memory;
                    self.cpu_usage = cpu;
                }
                Err(e) => {
                    eprintln!("System info error: {:?}", e);
                    match e {
                        SystemError::ProcessNotFound => {
                            self.process_memory = 0;
                            self.cpu_usage = 0.0;
                        }
                        SystemError::MemoryReadError => self.process_memory = 0,
                        SystemError::CpuReadError => self.cpu_usage = 0.0,
                    }
                }
            }
        }
    }
}