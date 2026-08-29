# Manual Testing Guide - Iori The Apacher

This document outlines manual testing procedures for features that cannot be fully automated in the current environment due to external dependencies (Tor, I2P, Freenet, Proxychains) or interactive nature (CLI shell).

## 1. Tor Network Launch
1. Ensure `tor` is installed on the host system.
2. Launch a server: `target/release/iori-the-apacher server launch --subdomain test-onion --port 8080 --protocol tor --crypto pfe969`.
3. Verify that a hidden service directory is created in `./services/tor/test-onion/`.
4. Verify `tor` process is running with the correct configuration.

## 2. I2P & Freenet
1. Ensure `i2p-router` is installed.
2. Launch a server: `target/release/iori-the-apacher server launch --subdomain test-i2p --port 8081 --protocol i2p --crypto pfe969`.
3. Verify the I2P router is active and tunnel parameters are configured.

## 3. Proxychains
1. Ensure `proxychains` and a SOCKS5 proxy (e.g. TOR) are running.
2. Launch with proxychains: `target/release/iori-the-apacher server launch --subdomain test-proxy --port 8082 --protocol tcp --crypto aes --proxychains true`.
3. Monitor outbound traffic using `tcpdump` to verify traffic is routed through the configured proxy.

## 4. Forensic Telemetry Analytics
1. Generate forensic hits from varied sources (bots, Tor exit nodes, local range).
2. Run `forensic list` and verify anomaly flags and risk scores are calculated according to the analytics logic.

## 5. Interactive Shell & History
1. Start shell: `target/release/iori-the-apacher shell`.
2. Execute OS commands: `ls -la`, `pwd`. Verify output is printed to the console.
3. Run multiple commands. Exit the shell (`exit`).
4. Re-start the shell: `target/release/iori-the-apacher shell`.
5. Press the up arrow key. Verify that the previous commands are retrieved from `.iori_history`.

## 6. Security Testing
1. Test command injection protection in OS execution:
   - Attempt: `ls | grep foo` (Should result in "command not found" for `grep` or an error, proving pipe is not interpreted by a shell).
   - Attempt: `ls > test.txt` (Should result in "command not found" for `>` file).
