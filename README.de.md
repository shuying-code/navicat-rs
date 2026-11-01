<p align="center">🌐 <b>Sprache</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# navicat-rs

Ein **plattformübergreifender Datenbank-Management-Desktop-Client als Einzeldatei**, geschrieben mit **Rust + egui/eframe**, UI und Workflow inspiriert von Navicat Premium 18.
**Alle Funktionen sind kostenlos und offen: kein Login, keine Testversion, keine Lizenz-/Rechtebeschränkungen.**

<p>
  <img alt="CI" src="https://github.com/shuying-code/navicat-rs/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/navicat-rs/actions/workflows/build.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.75%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/navicat-rs?display_name=tag">
</p>

> **Local-first** unabhängige Implementierung ohne Cloud-Verbindung, Login, Abo, Testversion oder Telemetrie. **Alle Funktionen sind für jeden bedingungslos verfügbar.**

![Willkommen](assets/screenshot-welcome.png)

---

## Warum „unbegrenzt"

Die bezahlten/Login-Beschränkungen von Navicat betreffen vor allem **Cloud-/Abo**-Funktionen (Cloud-Sync, Team-Zusammenarbeit, Cloud-Automatisierung usw.). Dieses Tool implementiert nur **cloud-unabhängige lokale** Datenbankverwaltung und benötigt daher kein Konto, keine Zahlung und kein Kontingent. Konstruktionsbedingt sind Login-/Abo-/Lizenzgrenzen entfernt; alle lokalen Funktionen sind bedingungslos verfügbar.

---

## Funktionen

- **Tabellen**: seitenweises Durchsuchen, Filtern, Inline-Bearbeitung, Zeilen hinzufügen/löschen, Sortieren (Klick auf Kopfzeile), Export.
- **SQL-Abfragen**: Mehrfachanweisungen, `Ctrl+Enter`, Mehrfach-Grid-Ergebnisse, Dauer / Zeilenanzahl, Explain-Planbaum, SQL-Formatierung, SQL-Outline / Refactoring-Hinweise, Abfrageverlauf.
- **Design & Modellierung**: Tabellendesigner (Spaltenbearbeitung → DDL), ER-Diagramm, Query-Builder, Modelldesigner (konzeptionell/logisch/physisch · Reverse-Engineering · Vergleich · Sync).
- **Datenbewegung**: Datentransfer (Spaltenmapping + WHERE), Daten-/Struktur-Sync, Import-Assistent (CSV/JSON/SQL), Export-Assistent (CSV/JSON/SQL/HTML), Data Pump.
- **Betrieb & Extras**: Backup/Restore (inkl. In-Memory-DB `VACUUM INTO`), Server-Monitor, Profiling, Schema-Analyse, Datengenerierung, Maskierung, Diagramme, Datenwörterbuch, Berichte.
- **Objekte & Suche**: Objektverwaltung (Index/Trigger/View), Suche in der Datenbank (spaltenwertübergreifende Suche über alle Tabellen), Zoom des Objektbaums und unabhängiges Einklappen von Gruppen.
- **Sonstiges**: AI-Assistent, Automatisierung / geplante Aufgaben, Code-Snippets, Benutzer & Rechte (alles offen), Einstellungen (Multitheme / Baum-Zoom / Auto-Commit / Löschbestätigung).
- **Multitheme**: hell / dunkel / Mitternacht / Wald / Sand / Graphit; Text von Body und Buttons bleibt in jedem Theme kontrastreich lesbar.

## GUI

- **Reines Rust + egui/eframe**: Einzeldatei-exe, kein WebView, Doppelklick zum Start; eingebettete CJK-Schriften, keine fehlenden Glyphen.
- **Drei-Zonen-Layout**: Menüleiste / Symbolleiste / Registerkarten / links Verbindung & Objektbaum / rechts Funktionsseiten / Statusleiste.
- **Objektbaum**: Verbindung → Tabellen / Views / Backups / Modelle / Benutzer; zoombar (`− 100% ＋`, `Ctrl+Mausrad`, Einstellungs-Slider), einklappbare Gruppen.
- **Mehrere Tabs**: Tabellen, Abfragen, Design, ER, Builder und Tool-Seiten parallel geöffnet.
- **Multitheme**: über `Ansicht → Theme` oder `Einstellungen → Theme`, Auswahl wird gespeichert.

## Unterstützte Plattformen

| Plattform | Status | Hinweis |
|---|---|---|
| Windows x64 | ✅ gebaut/verifiziert | MSVC-Toolchain empfohlen; fertige exe in Releases |
| macOS (Intel / Apple Silicon) | ✅ CI-Build (universal) | Universal-Binary via `build.yml` |
| Linux (X11 / Wayland) | ✅ CI-Build | x64-Archiv via `build.yml` (System-GUI-Abhängigkeiten nötig) |

## Build

Erfordert Rust 1.75+ (stable empfohlen).

```bash
cargo build --release
```

### Windows (MSVC)

```powershell
# Benötigt Visual Studio Build Tools (C++ und Windows SDK)
cargo build --release
```

### macOS

```bash
xcode-select --install   # ggf. beim ersten Mal nötig
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

## Vorgefertigte / plattformübergreifende Artefakte

- **Releases**: <https://github.com/shuying-code/navicat-rs/releases> (Windows x64 exe/zip, macOS universal tar.gz, Linux x64 tar.gz; erzeugt von `build.yml`).
- **CI-Matrix**: `.github/workflows/ci.yml` baut und führt `--selftest` auf `ubuntu-latest / macos-latest / windows-latest` aus.
- **Container-Image (GHCR)**:

```bash
docker pull ghcr.io/shuying-code/navicat-rs:latest
docker run --rm ghcr.io/shuying-code/navicat-rs:latest --selftest
```

## Verwendung

```bash
navicat-rs              # startet die GUI
navicat-rs --selftest   # Headless-Selbsttest (SQLite-Engine + Schlüsselfunktionen, 22 Prüfungen)
```

Beim ersten Start wird automatisch eine „Beispieldatenbank (SQLite)" angelegt (4 Tabellen + 1 View + Demodaten). Doppelklick auf eine Tabelle im Objektbaum zum Durchsuchen; die obere Symbolleiste öffnet Abfrage / Tabellendesign / ER / Query-Builder / Transfer / Import / Export / Backup / Monitor / Diagramme / Profiling / AI und alle weiteren Seiten.

## Arbeitsbereich (lokale Konfiguration)

Konfiguration und Status (Verbindungen, Verlauf, Favoriten, Theme, Einstellungen) werden im Benutzerkonfigurationsverzeichnis jeder Plattform gespeichert:

| Plattform | Pfad |
|---|---|
| Windows | `%APPDATA%\navicat-rs\navicat-rs.json` |
| macOS | `~/Library/Application Support/navicat-rs/navicat-rs.json` |
| Linux | `$XDG_CONFIG_HOME/navicat-rs/navicat-rs.json` oder `~/.config/navicat-rs/navicat-rs.json` |

## Verifikation

- `--selftest` deckt Engine und Schlüsselfunktionen ab: **22 Prüfungen, alle bestanden**.
- CI baut + Selbsttest auf drei Plattformen (Windows / macOS / Ubuntu): **alles success**.
- 6 Themes per Screenshot auf Lesbarkeit von Body / fetten Titeln / Kopfzeilen / Button-Text geprüft.

## Datenbank-Backends

| Engine | Status |
|---|---|
| SQLite | **Vollständig real**: Verbindung, Abfragen, CRUD, DDL, Skripte, Fremdschlüssel, PRAGMA (`rusqlite` bundled, plattformübergreifend) |
| PostgreSQL | **Echtes Netzwerkprotokoll**: Verbindungstest + echte Abfragen (reiner Rust-`postgres`-Treiber) |
| MySQL / MariaDB / Oracle / SQL Server / MongoDB / Redis / Snowflake / Dameng / ODBC | Verbindungskonfiguration + Offline-Demo-Schema (UI/Workflow vollständig; Treiberschicht abstrahiert, erweiterbar nach `src/realdb.rs`) |

## Kommandozeilenargumente

| Argument | Beschreibung |
|---|---|
| (keines) | Startet die GUI |
| `--selftest` | Headless-Selbsttest, gibt `[PASS]/[FAIL]` aus und liefert einen Exit-Code (0 = alles bestanden) |

## Umgebungsvariablen (optional)

| Variable | Beschreibung |
|---|---|
| `NAVICAT_RS_THEME` | Theme beim Start: `light`/`dark`/`midnight`/`forest`/`sand`/`graphite` |
| `NAVICAT_RS_DEMO` | `1` öffnet Beispielabfragen und -tabellen; `all` öffnet alle Funktionsseiten (für Screenshots/Abnahme) |
| `NAVICAT_RS_TAB` | Mit demo: wechselt zum Tab, dessen Titel diese Teilzeichenfolge enthält |

## FAQ

**F: Brauche ich ein Navicat-Konto oder eine Lizenz?** Nein. Die App öffnet direkt die Haupt-UI; keine Login-/Aktivierungs-/Testhinweise.

**F: Wo werden die Daten gespeichert?** Siehe „Arbeitsbereich"; Windows `%APPDATA%\navicat-rs\navicat-rs.json`; die Datenbank selbst ist Ihre verbundene SQLite-Datei.

**F: Warum wird das macOS-Binary in der CI gebaut?** Diese Maschine ist Windows; das macOS-Universal-Binary wird von GitHub Actions `macos-latest` erzeugt (siehe `build.yml`).

**F: Kann eine Nicht-SQLite-Engine wirklich verbinden?** SQLite ist vollständig real und PostgreSQL nutzt das echte Netzwerkprotokoll; die übrigen sind Konfiguration + Offline-Demo-DB, erweiterbar nach `src/realdb.rs`.

## Feedback (Issues)

Nutze GitHub Issues: <https://github.com/shuying-code/navicat-rs/issues/new/choose>. Lies vor dem Einreichen den **[Feedback-Leitfaden](docs/FEEDBACK.md)**. Maintainer bearbeiten Issues in der Regel einmal pro Woche.

## Lizenz

[MIT](LICENSE)
