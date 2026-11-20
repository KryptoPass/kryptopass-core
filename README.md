# KryptoPass Core

Crates de Rust sin UI de KryptoPass: bóveda, OpenPGP, Sync, dispositivos,
navegador y pinentry. Las apps los usan por ruta desde un checkout hermano:

```
KryptoPass.org/
├── kryptopass-core/      este repo
└── kryptopass-desktop/   app KDE (CXX-Qt + QML)
```

`just` lista las recetas; `just verify` es la puerta completa.
