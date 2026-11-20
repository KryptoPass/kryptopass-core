# KryptoPass Core — guía de trabajo

Crates de Rust sin UI: bóveda, plantillas, almacén SQLite, OpenPGP con GnuPG,
búsqueda privada, identidad del dispositivo, pairing, peers, Sync, LAN/Internet,
navegador y pinentry. Lo usan las apps por ruta; hoy, Desktop
(`../kryptopass-desktop`, CXX-Qt + QML). La UI presenta y el core decide y
ejecuta: cifrado, persistencia, autorización de lectura e invalidación.

`kp-backend` es lo que salió de `kp-desktop` tal cual. Mientras devuelva textos
para personas, la UI instala al arrancar su traductor
(`i18n::set_translator`) y el normalizador de iconos
(`infrastructure::entry_icons::set_icon_normalizer`). El objetivo es que
devuelva resultados tipados y que esos enganches desaparezcan.

Android (`../kryptopass-android`) es un igual de Desktop, no su cliente. El
formato común está en `../kryptopass-desktop/docs/sin-revisar/sync/contrato.md`:
cualquier cambio de formato se registra ahí antes de implementarlo en un lado.

## Cómo trabajar

- Antes de editar, declara objetivo, archivos, plan y verificación. No amplíes el
  alcance sin aprobación.
- El agente no escribe ni ejecuta tests salvo que el usuario lo pida.
- Mientras editas, basta `cargo check`. Al terminar, corre una vez la puerta
  acotada a lo que tocaste: `cargo fmt --all --check` y
  `cargo clippy -p <crates> --all-targets -- -D warnings`. Si cambia la API que
  usa Desktop, compila también Desktop (`cargo check -p kp-desktop` allí).
- `just verify` lo corre el usuario.
- `just` es la referencia. Un chequeo nuevo lleva receta en el `justfile` y job
  en `bacon.toml`, no un comando suelto en un doc.
- Commits: un cambio coherente por commit, Conventional Commits, mensaje en
  imperativo que describa el resultado. Skill `commit-quality`.
- Revisor `rust-comment-quality`: solo lee y reporta. Skills en `.agents/skills/`.

## Git: comandos peligrosos

- Antes de cualquier comando de Git, ejecuta y lee `git status`, `git diff` y
  `git stash list`. Nunca actúes sobre el árbol de trabajo, el índice o el
  historial sin saber qué hay modificado, sin seguimiento o en staging.
- Prohibido sin petición explícita del usuario y sin haber revisado el estado:
  `git reset --hard`, `git reset` que descarte cambios, `git checkout -- .`,
  `git checkout .`, `git restore` sobre cambios locales, `git clean` en
  cualquiera de sus formas, `git stash drop`/`clear`, `git branch -D`,
  `git push --force`, `git rebase`, `git commit --amend`, `git filter-branch`,
  `git filter-repo`, `git update-ref -d`, `git reflog expire`,
  `git gc --prune=now` y `git worktree remove --force`.
- Nunca descartar trabajo no commiteado, propio o ajeno, ni staged ni en stash.
  Ante cualquier duda, detente y pregunta.
- No usar `git add -A` ni `git add .`: añade ficheros concretos tras revisar el
  diff, para no stagear secretos, artefactos ni cambios ajenos.
- No cambiar de rama, hacer merge/pull/rebase, crear worktrees ni tocar stashes
  con cambios sin commitear sin aprobación.

## Nombres e identidad

- Todo ID público usa DNS inverso `org.kryptopass.*`.
- El `app-id` de `[workspace.metadata.kryptopass]` en `Cargo.toml` debe ser el
  mismo que el de Desktop: `kp-approval` lo lee al compilar (socket del pinentry,
  rutas del perfil). Pendiente: recibirlo en tiempo de ejecución.
- La palabra «lab» no se usa en código, UI ni disco (`kp-peers`, `peers-v1`).

## Datos durante el desarrollo

- No hay datos en producción. Un cambio de formato o de ruta se resuelve
  volviendo a vincular y empezando la bóveda de nuevo, sin migraciones; avísalo
  en el commit como `BREAKING CHANGE`.

## Identidad, confianza y claves

- Ninguna identidad se vuelve confiable por parsing, descubrimiento, una bandera
  de UI o una confirmación remota sin aprobación humana local (Noise XX + SAS).
  Segunda vía autorizada el 2026-11-11: una presentación firmada por un
  dispositivo ya confiable, recibida por su sesión autenticada, más aprobación
  local en ambos lados y comprobación de la identidad con Noise. Nunca automática.
- La identidad del dispositivo se crea al abrir la bóveda en un equipo sin perfil
  (`crates/kp-backend/src/vault_access.rs`). Nunca se regenera una identidad
  existente o corrupta. Si falta el trust de una identidad existente, puede
  recrearse vacío (no autoriza a nadie) registrándolo en el log.
- Borrar una vinculación no borra la bóveda ni las claves OpenPGP.
- La transferencia de claves privadas es una función de la app, no de debug.
  Exige confirmación local, la contraseña de GnuPG al exportar, aceptación en el
  otro dispositivo, almacén create-only y recibo con desafío.
- Sin secretos en texto plano como fallback ni en logs. Primitivas y protocolos
  con bibliotecas estándar, nunca implementados a mano.
- No versionar claves privadas de prueba en binarios, assets ni Base64: se
  generan localmente y se comparten entre builds debug con
  `KP_SYNTHETIC_FIXTURE_DIR`. Release no las incorpora. No regenerar en silencio
  una fixture existente.

## Conexión y red

- La recuperación de conexión no repite QR/SAS para identidades autorizadas y
  nunca genera identidades ni trust al reanudar. La desconexión humana detiene la
  recuperación; olvidar o resetear elimina su estado antes de borrar claves.
- Internet: el relay de señalización (`signal.kryptopass.org`, proyecto aparte)
  no es confiable. TURN usa credenciales temporales por petición firmada con una
  identidad existente; el token del proveedor solo vive en el servidor. Nunca
  incluir `KP_TURN_RELAY_ONLY` ni tokens del proveedor.
- Respetar las rutas del sistema y la VPN del usuario; no añadir interfaces
  físicas arbitrarias a ICE. «Exigir VPN» bloquea si no se puede verificar, con
  excepción LAN explícita y persistente. No modificar VPN ni firewall del sistema.
- No atribuir seguridad de producción a que algo compile o funcione: pairing,
  reconexión, Sync y transferencia de claves siguen sin revisión independiente.
