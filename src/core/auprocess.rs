use std::{fs, io, path::Path};

use anyhow::Result;
use sha2::{Digest, Sha256};

use super::process::Process;

fn game_assembly_dll_path(exe_path: &str) -> String {
    Path::new(exe_path)
        .parent()
        .unwrap()
        .join("GameAssembly.dll")
        .to_string_lossy()
        .into()
}

pub enum AUProcessError {
    ProcessNotFound,
    DllNotFound(io::Error),
}

pub struct AUProcess {
    process: Process,
    dll_hash: String,
}

impl AUProcess {
    pub fn new() -> Result<Self, AUProcessError> {
        let process = Process::find("Among Us.exe").ok_or(AUProcessError::ProcessNotFound)?;
        let dll_path = game_assembly_dll_path(&process.path());
        let mut hasher = Sha256::new();
        let dll_vec = fs::read(&dll_path).map_err(|x| AUProcessError::DllNotFound(x))?;
        hasher.update(dll_vec); // HEAVY!!
        Ok(Self {
            process,
            dll_hash: hex::encode_upper(hasher.finalize()),
        })
    }

    pub fn process(&self) -> &Process {
        &self.process
    }

    pub fn dll_hash(&self) -> &str {
        &self.dll_hash
    }
}
