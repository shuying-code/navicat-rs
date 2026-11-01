<p align="center">🌐 <b>Idioma</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# navicat-rs

Un **cliente de escritorio de gestión de bases de datos multiplataforma de un solo archivo**, escrito con **Rust + egui/eframe**, con UI y flujo de trabajo inspirados en Navicat Premium 18.
**Todas las funciones son gratuitas y abiertas: sin inicio de sesión, sin prueba, sin restricciones de licencia/permisos.**

<p>
  <img alt="CI" src="https://github.com/shuying-code/navicat-rs/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/navicat-rs/actions/workflows/build.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.75%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/navicat-rs?display_name=tag">
</p>

> Implementación independiente **local primero**, sin conexión a la nube ni inicio de sesión, suscripción, prueba o telemetría. **Todas las funciones están disponibles incondicionalmente para todos.**

![Bienvenida](assets/screenshot-welcome.png)

---

## Por qué «sin límites»

Las restricciones de pago/inicio de sesión de Navicat afectan sobre todo a funciones **en la nube / suscripción** (sincronización, colaboración de equipo, automatización en la nube, etc.). Esta herramienta implementa solo gestión **local independiente de la nube**, por lo que no requiere cuenta, pago ni cuota. Por diseño se eliminan los límites de inicio de sesión/suscripción/licencia; todas las funciones locales están disponibles sin condiciones.

---

## Funciones

- **Tablas**: navegación paginada, filtrado, edición en línea, agregar/eliminar filas, ordenar (clic en cabecera), exportar.
- **Consultas SQL**: ejecución de múltiples sentencias, `Ctrl+Enter`, resultados en múltiples cuadrículas, tiempo / número de filas, árbol de plan Explain, formateo SQL, esquema SQL / sugerencias de refactorización, historial de consultas.
- **Diseño y modelado**: diseñador de tablas (edición de columnas → DDL), diagrama ER, constructor de consultas, diseñador de modelos (conceptual/lógico/físico · ingeniería inversa · comparar · sincronizar).
- **Movimiento de datos**: transferencia (mapeo de columnas + WHERE), sincronización de datos/estructura, asistente de importación (CSV/JSON/SQL), asistente de exportación (CSV/JSON/SQL/HTML), data pump.
- **Operación y extras**: copia de seguridad/restauración (incl. BD en memoria `VACUUM INTO`), monitor de servidor, perfilado, análisis de esquema, generación de datos, enmascarado, gráficos, diccionario de datos, informes.
- **Objetos y búsqueda**: gestión de objetos (índice/trigger/vista), buscar en la base de datos (búsqueda de valores de columnas en todas las tablas), zoom del árbol de objetos y colapso independiente de grupos.
- **Otros**: asistente AI, automatización / tareas programadas, fragmentos de código, usuarios y permisos (todo abierto), ajustes (multitema / zoom del árbol / autocommit / confirmación de borrado).
- **Multitema**: claro / oscuro / medianoche / bosque / arena / grafito; el texto del cuerpo y de los botones se mantiene con alto contraste en cualquier tema.

## GUI

- **Rust puro + egui/eframe**: exe de un solo archivo, sin WebView, doble clic para ejecutar; fuentes CJK embebidas, sin cuadros de glifo faltante.
- **Diseño de tres zonas**: barra de menú / barra de herramientas / barra de pestañas / árbol de conexión y objetos a la izquierda / páginas de funciones a la derecha / barra de estado.
- **Árbol de objetos**: conexión → tablas / vistas / copias / modelos / usuarios; con zoom (`− 100% ＋`, `Ctrl+rueda`, control deslizante), grupos plegables.
- **Multipestaña**: tablas, consultas, diseño, ER, constructor y páginas de herramientas en paralelo.
- **Multitema**: cambia en `Ver → Tema` o `Ajustes → Tema`, la elección se guarda.

## Plataformas compatibles

| Plataforma | Estado | Notas |
|---|---|---|
| Windows x64 | ✅ compilado/verificado | Se recomienda toolchain MSVC; exe precompilado en Releases |
| macOS (Intel / Apple Silicon) | ✅ compilación CI (universal) | Binario universal generado por `build.yml` |
| Linux (X11 / Wayland) | ✅ compilación CI | Archivo x64 generado por `build.yml` (requiere dependencias GUI del sistema) |

## Compilación

Requiere Rust 1.75+ (se recomienda stable).

```bash
cargo build --release
```

### Windows (MSVC)

```powershell
# Requiere Visual Studio Build Tools (C++ y Windows SDK)
cargo build --release
```

### macOS

```bash
xcode-select --install   # puede requerirse la primera vez
cargo build --release
```

### Linux (Debian/Ubuntu)

```bash
sudo apt-get install -y build-essential pkg-config \
  libxkbcommon-dev libgtk-3-dev libgl1-mesa-dev \
  libx11-dev libxi-dev libxrandr-dev libxcursor-dev libxinerama-dev \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
cargo build --release
```

## Artefactos precompilados / multiplataforma

- **Releases**: <https://github.com/shuying-code/navicat-rs/releases> (Windows x64 exe/zip, macOS universal tar.gz, Linux x64 tar.gz; generado por `build.yml`).
- **Matriz CI**: `.github/workflows/ci.yml` compila y ejecuta `--selftest` en `ubuntu-latest / macos-latest / windows-latest`.
- **Imagen de contenedor (GHCR)**:

```bash
docker pull ghcr.io/shuying-code/navicat-rs:latest
docker run --rm ghcr.io/shuying-code/navicat-rs:latest --selftest
```

## Uso

```bash
navicat-rs              # inicia la GUI
navicat-rs --selftest   # autoprueba sin interfaz (motor SQLite + funciones clave, 22 comprobaciones)
```

En el primer inicio crea automáticamente una "Base de demostración (SQLite)" (4 tablas + 1 vista + datos de demo). Haz doble clic en una tabla del árbol de objetos para explorarla; la barra superior abre consulta / diseño de tabla / ER / constructor de consultas / transferencia / importación / exportación / copia de seguridad / monitor / gráficos / perfilado / AI y todas las demás páginas.

## Espacio de trabajo (config local)

La configuración y el estado (conexiones, historial, favoritos, tema, ajustes) se guardan en el directorio de configuración de usuario de cada plataforma:

| Plataforma | Ruta |
|---|---|
| Windows | `%APPDATA%\navicat-rs\navicat-rs.json` |
| macOS | `~/Library/Application Support/navicat-rs/navicat-rs.json` |
| Linux | `$XDG_CONFIG_HOME/navicat-rs/navicat-rs.json` o `~/.config/navicat-rs/navicat-rs.json` |

## Verificación

- `--selftest` cubre el motor y las funciones clave: **22 comprobaciones, todas pasan**.
- CI compila + autoprueba en tres plataformas (Windows / macOS / Ubuntu): **todo success**.
- 6 temas verificados con capturas para la legibilidad de cuerpo / títulos en negrita / cabeceras / texto de botones.

## Motores de base de datos

| Motor | Estado |
|---|---|
| SQLite | **Totalmente real**: conexión, consultas, CRUD, DDL, scripts, claves foráneas, PRAGMA (`rusqlite` bundled, multiplataforma) |
| PostgreSQL | **Protocolo de red real**: prueba de conexión + consultas reales (driver `postgres` en Rust puro) |
| MySQL / MariaDB / Oracle / SQL Server / MongoDB / Redis / Snowflake / Dameng / ODBC | Configuración de conexión + schema de demo offline (UI/flujo completos; capa de driver abstraída, ampliable según `src/realdb.rs`) |

## Argumentos de línea de comandos

| Argumento | Descripción |
|---|---|
| (ninguno) | Inicia la GUI |
| `--selftest` | Autoprueba sin interfaz, imprime `[PASS]/[FAIL]` y devuelve un código de salida (0 = todo pasa) |

## Variables de entorno (opcional)

| Variable | Descripción |
|---|---|
| `NAVICAT_RS_THEME` | Tema al iniciar: `light`/`dark`/`midnight`/`forest`/`sand`/`graphite` |
| `NAVICAT_RS_DEMO` | `1` abre consultas y tablas de ejemplo; `all` abre todas las páginas de funciones (para capturas/aceptación) |
| `NAVICAT_RS_TAB` | Con demo, cambia a la pestaña cuyo título contiene esta subcadena |

## Preguntas frecuentes (FAQ)

**P: ¿Necesito una cuenta o licencia de Navicat?** No. La app abre directamente la UI principal; sin avisos de inicio de sesión / activación / prueba.

**P: ¿Dónde se guardan los datos?** Ver "Espacio de trabajo"; Windows `%APPDATA%\navicat-rs\navicat-rs.json`; la base de datos es tu archivo SQLite conectado.

**P: ¿Por qué el binario de macOS se compila en CI?** Esta máquina es Windows; el binario universal de macOS lo produce GitHub Actions `macos-latest` (ver `build.yml`).

**P: ¿Un motor no-SQLite puede conectarse de verdad?** SQLite es totalmente real y PostgreSQL usa el protocolo de red real; los demás son configuración + BD de demo offline, ampliables según `src/realdb.rs`.

## Comentarios (Issues)

Usa GitHub Issues: <https://github.com/shuying-code/navicat-rs/issues/new/choose>. Lee la **[guía de comentarios](docs/FEEDBACK.md)** antes de enviar. Los mantenedores suelen atender los issues una vez por semana.

## Licencia

[MIT](LICENSE)
