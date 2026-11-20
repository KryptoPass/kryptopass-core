# KryptoPass Core — pipeline local (just + bacon)
#
#   just verify   -> puerta completa (antes de commitear)
#   bacon         -> chequeo continuo en fondo (job check)
#
# Secciones: Desarrollo, Pruebas, Puertas, Dispositivos y P2P, Integraciones

set shell := ["bash", "-uc"]

_default:
    @just --list --unsorted

#
# Desarrollo
#

# Compila (reenvía flags a cargo: `just build --release`).
[group('Desarrollo')]
build *args:
    cargo build --workspace {{ args }}

[group('Desarrollo')]
check:
    cargo check --workspace

# Formato Rust: debe pasar sin diff.
[group('Desarrollo')]
fmt:
    cargo fmt --all --check

# Aplica el mismo formato que comprueba verify.
[group('Desarrollo')]
format:
    cargo fmt --all

# Lints Rust como errores.
[group('Desarrollo')]
clippy:
    cargo clippy --workspace --all-targets -- -D warnings

# Vulnerabilidades conocidas en dependencias (cargo install cargo-audit).
# Las aceptadas a propósito están justificadas en .cargo/audit.toml.
[group('Desarrollo')]
audit:
    cargo audit

#
# Pruebas
#

[group('Pruebas')]
test:
    cargo test --workspace

# Reset explícito con Secret Service separado y archivos temporales, sin datos del usuario.
[group('Pruebas')]
test-dev-reset:
    cargo test -p kp-peers --test profile_reset dev_reset -- --ignored --nocapture

# TURN dataplane real opt-in: pipea un bundle temporal en stdin; no consulta el broker.
[group('Pruebas')]
turn-smoke:
    KP_TURN_SMOKE=1 cargo test -p kp-lan --test turn_smoke -- --ignored --nocapture

#
# Puertas
#

# Puerta completa.
[group('Puertas')]
verify: fmt clippy test

# Puerta acotada a protocolo/Sync/owner.
[group('Puertas')]
verify-sync-session:
    cargo fmt -p kp-protocol -p kp-sync -p kp-devices --check
    cargo clippy -p kp-protocol -p kp-sync -p kp-devices --all-targets -- -D warnings
    cargo test -p kp-protocol -p kp-sync -p kp-devices -p kp-pairing -p kp-transport

#
# Dispositivos y P2P
#

# Dos perfiles temporales, aprobación SAS simulada y reconexión; sin GPG ni red.
[group('Dispositivos y P2P')]
p2p-demo:
    cargo run -p kp-pairing --example pairing_demo

#
# Integraciones
#

# Proxy de navegador sin Qt. Desktop lo construye con su `just browser-build`.
[group('Integraciones')]
browser-build:
    cargo build -p kp-browser --bin kryptopass-browser-proxy

# Pinentry de aprobaciones. Desktop lo construye con su `just pinentry-build`.
[group('Integraciones')]
pinentry-build:
    cargo build -p kp-pinentry --bin kryptopass-pinentry
