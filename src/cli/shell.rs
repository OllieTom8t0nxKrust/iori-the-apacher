use std::process::Command;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use tokio::runtime::Runtime;
use crate::application::service::ApplicationService;

pub fn run_interactive_shell(_service: ApplicationService, _rt: &Runtime) -> Result<(), String> {
    let mut rl = DefaultEditor::new().map_err(|e| e.to_string())?;
    let _ = rl.load_history(".iori_history");

    println!("==========================================================================================");
    println!("  Iori the APACHER - Interactive REPL Shell & Advanced Routing Suite (Tor/I2P/Freenet/PQC)");
    println!("==========================================================================================");
    
    loop {
        let readline = rl.readline("iori-apacher> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.is_empty() { continue; }
                
                let cmd = parts[0].to_lowercase();
                
                match cmd.as_str() {
                    "exit" | "quit" => break,
                    "help" => {
                        println!("1. OS Commands: 'ls', 'pwd', etc. run natively.");
                        println!("2. App Commands: tunnel, forensic, crypto, server.");
                    }
                    _ => execute_system_command(&parts),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(e) => { println!("Error: {:?}", e); break; }
        }
    }
    let _ = rl.save_history(".iori_history");
    Ok(())
}

fn execute_system_command(parts: &[&str]) {
    let output = Command::new(parts[0]).args(&parts[1..]).output();
    match output {
        Ok(o) => {
            if !o.stdout.is_empty() { print!("{}", String::from_utf8_lossy(&o.stdout)); }
            if !o.stderr.is_empty() { print!("{}", String::from_utf8_lossy(&o.stderr)); }
        }
        Err(e) => println!("Failed to execute command: {}", e),
    }
}
