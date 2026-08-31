# Manual Testing Guide - IORI THE APACHER // Enterprise Post-Quantum Secure Tunneling & Cryptographic Suite

This document outlines manual testing procedures for features that require external network daemons (Tor, I2P, Proxychains) or interactive CLI shell execution.

## 1. Tor Network Launch Verification
1. Verify `tor` is installed on the host system.
2. Launch a secure Tor onion server: 
   ```bash
   target/release/iori-the-apacher server launch --subdomain test-onion --port 8080 --protocol tor --crypto pfe969
   ```
3. Confirm hidden service directory creation under `./services/tor/test-onion/`.
4. Verify the active Tor daemon process configuration.

## 2. I2P & Freenet Protocol Testing
1. Verify `i2p-router` is active.
2. Launch an I2P server tunnel:
   ```bash
   target/release/iori-the-apacher server launch --subdomain test-i2p --port 8081 --protocol i2p --crypto pfe969
   ```
3. Confirm router tunnel bindings and parameters.

## 3. Proxychains & Multi-Hop Chaining
1. Ensure `proxychains` and SOCKS5 proxy daemon are running.
2. Launch server with proxychains and multi-hop relay chaining:
   ```bash
   target/release/iori-the-apacher server launch --subdomain test-proxy --port 8082 --protocol tcp --crypto aes --hops "relay1,relay2" --proxychains true
   ```
3. Monitor outbound traffic using network diagnostics to verify proxy encapsulation.

## 4. Interactive Shell REPL & History Testing
1. Start the REPL shell:
   ```bash
   target/release/iori-the-apacher shell
   ```
2. Execute system commands (`ls -la`, `pwd`) and verify output formatting.
3. Exit the shell (`exit`).
4. Re-launch the shell and press the `Up` arrow key to verify persistent history retrieval from `.iori_history`.

## 6. Frontend UI Responsiveness & Interactive Logo
1. Navigate to the web management console.
2. Verify the sidebar is fully expanded.
3. Click the Caboclo-Iori logo:
   - Confirm the sidebar collapses to a narrow width.
   - Confirm the logo transforms into a skull icon with glowing red eyes.
4. Click the skull logo again:
   - Confirm the sidebar expands to full width.
   - Confirm the logo returns to the original photo.
5. Resize the browser window to mobile dimensions (e.g., < 768px):
   - Confirm the layout remains usable and fits within the viewport.

