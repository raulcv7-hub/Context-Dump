# flux-context (CLI)

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Context** es un motor de ingesta de alto rendimiento diseñado para transformar repositorios complejos y documentación técnica en un contexto unificado, optimizado para LLMs (ChatGPT, Claude, Llama 3).

## Características Principales

*   **Rendimiento Multihilo**: Procesamiento paralelo masivo mediante `Rayon`.
*   **Persistencia de Estado**: Recuerda tu última configuración (formato, filtros, minificación) automáticamente.
*   **Modo Inteligente**: Si se invoca con argumentos (ej: `context .`), ejecuta la última configuración guardada de forma instantánea.
*   **Búsqueda Interactiva**: Filtra el árbol de archivos en tiempo real en la TUI presionando `/`.
*   **Smart Ignore**: Heurísticas dinámicas para omitir artefactos pesados (>1MB) y binarios conocidos.
*   **Parsers de Alta Fidelidad**: Soporte avanzado para PDF (con detección de tablas y código), DOCX, Excel y Texto.

## Uso Estratégico

### 1. Flujo Interactivo (Naked Call)
Ejecuta el comando sin argumentos para abrir la TUI:
```bash
context
```
*Configura tus preferencias, busca archivos con `/` y confirma con `Enter`. Esta configuración se volverá tu predeterminada.*

## Interfaz TUI (Panel de Control)

| Tecla | Acción |
| :--- | :--- |
| `/` | **Activar Búsqueda** (filtra el árbol en tiempo real). |
| `Espacio` | Seleccionar / Deseleccionar (aplica recursivamente). |
| `Enter` | **Confirmar y Procesar**. |
| `c` | Alternar **Clipboard** (ON/OFF). |
| `m` | Alternar **Minificación** (ON/OFF). |
| `o` | Alternar **Archivo de Salida**. |
| `f` | Ciclar **Formato** (XML -> MD -> JSON -> TXT). |
| `Esc` | Limpiar búsqueda / Salir. |

## Opciones del Binario

| Flag | Descripción |
| :--- | :--- |
| `-o, --output <FILE>` | Ruta de salida. Detecta formato por extensión. |
| `-s, --stdout` | Vuelca a terminal (desactiva TUI y sobreescribe archivo). |
| `-c, --clip` | Copia el resultado al portapapeles. |
| `-m, --minify` | Reduce el peso eliminando espacios/líneas vacías. |
| `-S, --smart-ignore` | Activa/Desactiva heurísticas de ruido (def: true). |
| `-e, --extensions` | Lista blanca de extensiones (ej: `rs,py,ts`). |
| `-x, --exclude` | Lista negra de extensiones. |
| `-i, --include-path` | Filtro de inclusión por cadena en ruta. |
| `-X, --exclude-path` | Filtro de exclusión por cadena en ruta. |
| `-I, --interactive` | Fuerza el modo TUI ignorando otros flags. |
| `-v, --verbose` | Nivel de logs (`-v` INFO, `-vv` DEBUG). |

## Arquitectura

El proyecto sigue una **Arquitectura Hexagonal** estricta:

```text
src/
├── core/           # Dominio (Config, Persistencia, Minifier)
├── ports/          # Interfaces (Traits)
├── adapters/       # Implementaciones técnicas (Scanner, Parsers, Output)
│   ├── fs_scanner/ # Smart Ignore y filtrado
│   └── parsers/    # PDF con Layout Analysis, Excel, Word
└── ui/             # TUI reactiva (Ratatui)
```
