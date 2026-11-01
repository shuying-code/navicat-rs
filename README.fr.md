<p align="center">🌐 <b>Langue</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# navicat-rs

Un **client de bureau de gestion de bases de données multiplateforme mono-fichier**, écrit avec **Rust + egui/eframe**, dont l'UI et le flux de travail s'inspirent de Navicat Premium 18.
**Toutes les fonctionnalités sont gratuites et ouvertes : sans connexion, sans essai, sans restriction de licence/permission.**

<p>
  <img alt="CI" src="https://github.com/shuying-code/navicat-rs/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/navicat-rs/actions/workflows/build.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.75%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/navicat-rs?display_name=tag">
</p>

> Implémentation indépendante **local d'abord**, sans connexion cloud ni login, abonnement, essai ou télémétrie. **Toutes les fonctionnalités sont disponibles sans condition pour tous.**

![Bienvenue](assets/screenshot-welcome.png)

---

## Pourquoi « sans limites »

Les contraintes payantes/connexion de Navicat concernent surtout les fonctionnalités **cloud / abonnement** (synchronisation cloud, collaboration d'équipe, automatisation cloud, etc.). Cet outil n'implémente que la gestion **locale indépendante du cloud**, donc aucun compte, aucun paiement, aucun quota. Par conception, les limites de connexion/abonnement/licence sont supprimées ; toutes les fonctions locales sont disponibles sans condition.

---

## Fonctionnalités

- **Tables** : navigation paginée, filtrage, édition en ligne, ajout/suppression de lignes, tri (clic en-tête), export.
- **Requêtes SQL** : exécution multi-instructions, `Ctrl+Enter`, résultats multi-grilles, durée / nombre de lignes, arbre de plan Explain, formatage SQL, plan SQL / suggestions de refactorisation, historique des requêtes.
- **Conception et modélisation** : concepteur de tables (édition de colonnes → DDL), diagramme ER, constructeur de requêtes, concepteur de modèles (conceptuel/logique/physique · ingénierie inverse · comparaison · synchronisation).
- **Déplacement de données** : transfert (mappage de colonnes + WHERE), synchronisation données/structure, assistant d'import (CSV/JSON/SQL), assistant d'export (CSV/JSON/SQL/HTML), data pump.
- **Exploitation et extras** : sauvegarde/restauration (y compris BD en mémoire `VACUUM INTO`), moniteur de serveur, profilage, analyse de schéma, génération de données, masquage, graphiques, dictionnaire de données, rapports.
- **Objets et recherche** : gestion des objets (index/trigger/vue), recherche dans la base (recherche de valeurs de colonnes sur toutes les tables), zoom de l'arbre d'objets et repli de groupes.
- **Autres** : assistant AI, automatisation / tâches planifiées, extraits de code, utilisateurs et permissions (tout ouvert), paramètres (multithème / zoom d'arbre / auto-commit / confirmation de suppression).
- **Multithème** : clair / sombre / minuit / forêt / sable / graphite ; le texte du corps et des boutons reste lisible avec un fort contraste dans tous les thèmes.

## GUI

- **Rust pur + egui/eframe** : exe mono-fichier, sans WebView, lancement par double-clic ; polices CJK intégrées, pas de carrés manquants.
- **Mise en page en trois zones** : barre de menus / barre d'outils / barre d'onglets / arbre de connexion et d'objets à gauche / pages de fonctions à droite / barre d'état.
- **Arbre d'objets** : connexion → tables / vues / sauvegardes / modèles / utilisateurs ; zoomable (`− 100% ＋`, `Ctrl+molette`, curseur des paramètres), groupes repliables.
- **Multi-onglets** : tables, requêtes, conception, ER, constructeur et pages d'outils ouvertes en parallèle.
- **Multithème** : via `Affichage → Thème` ou `Paramètres → Thème`, choix mémorisé.

## Plateformes prises en charge

| Plateforme | État | Notes |
|---|---|---|
| Windows x64 | ✅ compilé/vérifié | Toolchain MSVC recommandé ; exe précompilé dans les Releases |
| macOS (Intel / Apple Silicon) | ✅ build CI (universal) | Binaire universel produit par `build.yml` |
| Linux (X11 / Wayland) | ✅ build CI | Archive x64 produite par `build.yml` (dépendances GUI système requises) |

## Compilation

Requiert Rust 1.75+ (stable recommandé).

```bash
cargo build --release
```

### Windows (MSVC)

```powershell
# Nécessite Visual Studio Build Tools (C++ et Windows SDK)
cargo build --release
```

### macOS

```bash
xcode-select --install   # peut être nécessaire la première fois
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

## Artefacts précompilés / multiplateformes

- **Releases** : <https://github.com/shuying-code/navicat-rs/releases> (Windows x64 exe/zip, macOS universal tar.gz, Linux x64 tar.gz ; produit par `build.yml`).
- **Matrice CI** : `.github/workflows/ci.yml` compile et exécute `--selftest` sur `ubuntu-latest / macos-latest / windows-latest`.
- **Image conteneur (GHCR)** :

```bash
docker pull ghcr.io/shuying-code/navicat-rs:latest
docker run --rm ghcr.io/shuying-code/navicat-rs:latest --selftest
```

## Utilisation

```bash
navicat-rs              # lance l'interface graphique
navicat-rs --selftest   # autotest sans interface (moteur SQLite + fonctions clés, 22 vérifications)
```

Au premier lancement, une « base de démonstration (SQLite) » est créée automatiquement (4 tables + 1 vue + données de démo). Double-cliquez sur une table dans l'arbre d'objets pour la parcourir ; la barre d'outils supérieure ouvre requête / conception de table / ER / constructeur de requêtes / transfert / import / export / sauvegarde / moniteur / graphiques / profilage / AI et toutes les autres pages.

## Espace de travail (config locale)

La configuration et l'état (connexions, historique, favoris, thème, paramètres) sont stockés dans le répertoire de configuration utilisateur de chaque plateforme :

| Plateforme | Chemin |
|---|---|
| Windows | `%APPDATA%\navicat-rs\navicat-rs.json` |
| macOS | `~/Library/Application Support/navicat-rs/navicat-rs.json` |
| Linux | `$XDG_CONFIG_HOME/navicat-rs/navicat-rs.json` ou `~/.config/navicat-rs/navicat-rs.json` |

## Vérification

- `--selftest` couvre le moteur et les fonctions clés : **22 vérifications, toutes réussies**.
- CI compile + autotest sur trois plateformes (Windows / macOS / Ubuntu) : **tout success**.
- 6 thèmes vérifiés par captures pour la lisibilité du corps / des titres en gras / des en-têtes / du texte des boutons.

## Moteurs de base de données

| Moteur | État |
|---|---|
| SQLite | **Totalement réel** : connexion, requêtes, CRUD, DDL, scripts, clés étrangères, PRAGMA (`rusqlite` bundled, multiplateforme) |
| PostgreSQL | **Protocole réseau réel** : test de connexion + requêtes réelles (driver `postgres` en Rust pur) |
| MySQL / MariaDB / Oracle / SQL Server / MongoDB / Redis / Snowflake / Dameng / ODBC | Config de connexion + schéma de démo hors ligne (UI/flux complets ; couche driver abstraite, extensible via `src/realdb.rs`) |

## Arguments de ligne de commande

| Argument | Description |
|---|---|
| (aucun) | Lance l'interface graphique |
| `--selftest` | Autotest sans interface, imprime `[PASS]/[FAIL]` et renvoie un code de sortie (0 = tout réussi) |

## Variables d'environnement (facultatif)

| Variable | Description |
|---|---|
| `NAVICAT_RS_THEME` | Thème au démarrage : `light`/`dark`/`midnight`/`forest`/`sand`/`graphite` |
| `NAVICAT_RS_DEMO` | `1` ouvre des requêtes et tables d'exemple ; `all` ouvre toutes les pages de fonctions (captures/acceptation) |
| `NAVICAT_RS_TAB` | Avec demo, bascule vers l'onglet dont le titre contient cette sous-chaîne |

## FAQ

**Q : Faut-il un compte ou une licence Navicat ?** Non. L'app ouvre directement l'UI principale ; aucune invite de connexion / activation / essai.

**Q : Où sont stockées les données ?** Voir « Espace de travail » ; Windows `%APPDATA%\navicat-rs\navicat-rs.json` ; la base elle-même est votre fichier SQLite connecté.

**Q : Pourquoi le binaire macOS est-il compilé en CI ?** Cette machine est sous Windows ; le binaire universel macOS est produit par GitHub Actions `macos-latest` (voir `build.yml`).

**Q : Un moteur non-SQLite peut-il réellement se connecter ?** SQLite est totalement réel et PostgreSQL utilise le vrai protocole réseau ; les autres sont config + BD de démo hors ligne, extensibles via `src/realdb.rs`.

## Retour d'information (Issues)

Utilisez GitHub Issues : <https://github.com/shuying-code/navicat-rs/issues/new/choose>. Lisez le **[guide de retour](docs/FEEDBACK.md)** avant de soumettre. Les mainteneurs traitent généralement les issues une fois par semaine.

## Licence

[MIT](LICENSE)
