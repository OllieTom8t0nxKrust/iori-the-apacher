# Interactive CLI Guide - Iori The Apacher

The `iori-the-apacher` interactive shell (REPL) is a powerful tool designed for cybersecurity investigations and forensic analysis.

## Features

### 1. Command History
The shell automatically saves and loads your command history to a file named `.iori_history` in the project root directory. Use the up and down arrow keys to navigate through your previous commands.

### 2. OS Command Execution
You can execute native Linux commands directly from the `iori-apacher>` prompt.

*   **Examples**:
    *   `ls -la` (List files in the current directory)
    *   `pwd` (Print working directory)
    *   `whoami` (Current user)
    *   `ps aux | grep tor` (Note: Piped commands like `|` or redirects `>` are **NOT supported** to ensure system security.)

### 3. Security Design
The shell executes OS commands by spawning them as direct child processes (`std::process::Command`), NOT by invoking a shell (like `/bin/sh -c` or `/bin/bash -c`). This design is intentional and critical for security:

*   **No Shell Injection**: Malicious payloads involving shell metacharacters (pipes `|`, redirections `>`,`<`, `&&`, `;`) are treated as literal arguments to the binary being executed, rather than being parsed by a shell.
*   **Restricted Execution**: Only binary executables can be run. Shell-builtin commands (like `cd`, `export`, etc.) must be handled as binary executables (`/usr/bin/cd` if available).

## Troubleshooting OS Commands
If a command fails to execute, ensure:
1. The executable exists in your system `$PATH`.
2. You are not attempting to use shell built-ins or syntax (e.g., pipes or redirects).
3. The command does not require an interactive TTY input (like `vi` or `nano`).
