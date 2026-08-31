# Interactive CLI Guide - IORI THE APACHER // Enterprise Post-Quantum Secure Tunneling & Cryptographic Suite

The `iori-the-apacher` interactive REPL shell is a high-performance, secure operational console designed for network tunneling, post-quantum cryptographic operations, and secure server management.

## Key Features & Capabilities

### 1. Persistent Command History
The shell automatically manages command history across sessions, storing entries in `.iori_history` within the project root directory. Use arrow keys (`Up` / `Down`) to navigate through previous commands.

### 2. Direct OS Command Execution
Execute native Linux system binaries directly from the `iori-apacher>` prompt.

*   **Supported Examples**:
    *   `ls -la` (List directory contents)
    *   `pwd` (Print working directory)
    *   `whoami` (Current authenticated user)
    *   `ps aux` (Process snapshot)
    *   *(Note: Shell redirection `>` and piping `|` are intentionally disabled for security).*

### 3. Security-First Execution Architecture
The interactive shell executes system commands by spawning direct child processes (`std::process::Command`), bypassing shell interpreters (`/bin/sh` or `/bin/bash`):
*   **Immune to Shell Injection**: Metacharacters (`|`, `>`, `<`, `&&`, `;`) are treated as literal argument strings rather than evaluated by a shell parser.
*   **Controlled Scope**: Only explicit binaries found in system `$PATH` are executable.

## Troubleshooting & Built-In Help
- Use the built-in `help` command inside the REPL shell for instant command syntax reference.
- Ensure invoked binaries exist in your system `$PATH` and do not require interactive TTY text editors (`vi`, `nano`).
