<p align="center">🌐 <b>Idioma</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# navicat-rs

Um **cliente de desktop para gerenciamento de bancos de dados multiplataforma de arquivo único**, escrito com **Rust + egui/eframe**, com UI e fluxo de trabalho inspirados no Navicat Premium 18.
**Todos os recursos são gratuitos e abertos: sem login, sem teste, sem restrições de licença/permissão.**

<p>
  <img alt="CI" src="https://github.com/shuying-code/navicat-rs/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/navicat-rs/actions/workflows/build.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.75%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/navicat-rs?display_name=tag">
</p>

> Implementação independente **local primeiro**, sem conexão com a nuvem nem login, assinatura, teste ou telemetria. **Todos os recursos estão disponíveis incondicionalmente para todos.**

![Bem-vindo](assets/screenshot-welcome.png)

---

## Por que «sem limites»

As restrições pagas/de login do Navicat afetam principalmente recursos **de nuvem / assinatura** (sincronização na nuvem, colaboração em equipe, automação na nuvem etc.). Esta ferramenta implementa apenas gerenciamento **local independente da nuvem**, portanto não exige conta, pagamento nem cota. Por design, os limites de login/assinatura/licença foram removidos; todos os recursos locais estão disponíveis sem condições.

---

## Recursos

- **Tabelas**: navegação paginada, filtragem, edição inline, adicionar/excluir linhas, ordenar (clique no cabeçalho), exportar.
- **Consultas SQL**: execução de múltiplas instruções, `Ctrl+Enter`, resultados em múltiplas grades, tempo / nº de linhas, árvore de plano Explain, formatação SQL, esboço SQL / sugestões de refatoração, histórico de consultas.
- **Design e modelagem**: designer de tabelas (edição de colunas → DDL), diagrama ER, construtor de consultas, designer de modelos (conceitual/lógico/físico · engenharia reversa · comparação · sincronização).
- **Movimentação de dados**: transferência (mapeamento de colunas + WHERE), sincronização de dados/estrutura, assistente de importação (CSV/JSON/SQL), assistente de exportação (CSV/JSON/SQL/HTML), data pump.
- **Operação e extras**: backup/restauração (incl. BD em memória `VACUUM INTO`), monitor de servidor, profiling, análise de esquema, geração de dados, mascaramento, gráficos, dicionário de dados, relatórios.
- **Objetos e busca**: gerenciamento de objetos (índice/trigger/view), busca no banco (busca de valores de colunas em todas as tabelas), zoom da árvore de objetos e recolhimento de grupos.
- **Outros**: assistente AI, automação / tarefas agendadas, snippets de código, usuários e permissões (tudo aberto), configurações (multitema / zoom da árvore / auto-commit / confirmação de exclusão).
- **Multitema**: claro / escuro / meia-noite / floresta / areia / grafite; o texto do corpo e dos botões permanece com alto contraste e legível em qualquer tema.

## GUI

- **Rust puro + egui/eframe**: exe de arquivo único, sem WebView, duplo clique para executar; fontes CJK embutidas, sem quadrados de glifo ausente.
- **Layout em três regiões**: barra de menus / barra de ferramentas / barra de abas / árvore de conexão e objetos à esquerda / páginas de recursos à direita / barra de status.
- **Árvore de objetos**: conexão → tabelas / views / backups / modelos / usuários; com zoom (`− 100% ＋`, `Ctrl+roda`, slider nas configurações), grupos recolhíveis.
- **Multia-bas**: tabelas, consultas, design, ER, construtor e páginas de ferramentas abertas em paralelo.
- **Multitema**: troque em `Exibir → Tema` ou `Configurações → Tema`, escolha persistida.

## Plataformas suportadas

| Plataforma | Status | Observação |
|---|---|---|
| Windows x64 | ✅ compilado/verificado | Toolchain MSVC recomendado; exe pré-compilado nas Releases |
| macOS (Intel / Apple Silicon) | ✅ build CI (universal) | Binário universal gerado por `build.yml` |
| Linux (X11 / Wayland) | ✅ build CI | Arquivo x64 gerado por `build.yml` (requer dependências GUI do sistema) |

## Compilação

Requer Rust 1.75+ (stable recomendado).

```bash
cargo build --release
```

### Windows (MSVC)

```powershell
# Requer Visual Studio Build Tools (C++ e Windows SDK)
cargo build --release
```

### macOS

```bash
xcode-select --install   # pode ser necessário na primeira vez
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

## Artefatos pré-compilados / multiplataforma

- **Releases**: <https://github.com/shuying-code/navicat-rs/releases> (Windows x64 exe/zip, macOS universal tar.gz, Linux x64 tar.gz; gerado por `build.yml`).
- **Matriz CI**: `.github/workflows/ci.yml` compila e executa `--selftest` em `ubuntu-latest / macos-latest / windows-latest`.
- **Imagem de contêiner (GHCR)**:

```bash
docker pull ghcr.io/shuying-code/navicat-rs:latest
docker run --rm ghcr.io/shuying-code/navicat-rs:latest --selftest
```

## Uso

```bash
navicat-rs              # inicia a GUI
navicat-rs --selftest   # autoteste sem interface (motor SQLite + recursos-chave, 22 verificações)
```

No primeiro início, cria automaticamente um "Banco de demonstração (SQLite)" (4 tabelas + 1 view + dados de demo). Dê duplo clique em uma tabela na árvore de objetos para navegar; a barra de ferramentas superior abre consulta / design de tabela / ER / construtor de consultas / transferência / importação / exportação / backup / monitor / gráficos / profiling / AI e todas as demais páginas.

## Área de trabalho (config local)

A configuração e o estado (conexões, histórico, favoritos, tema, configurações) ficam no diretório de configuração do usuário de cada plataforma:

| Plataforma | Caminho |
|---|---|
| Windows | `%APPDATA%\navicat-rs\navicat-rs.json` |
| macOS | `~/Library/Application Support/navicat-rs/navicat-rs.json` |
| Linux | `$XDG_CONFIG_HOME/navicat-rs/navicat-rs.json` ou `~/.config/navicat-rs/navicat-rs.json` |

## Verificação

- `--selftest` cobre o motor e os recursos-chave: **22 verificações, todas passam**.
- CI compila + autoteste em três plataformas (Windows / macOS / Ubuntu): **tudo success**.
- 6 temas verificados por captura quanto à legibilidade de corpo / títulos em negrito / cabeçalhos / texto de botões.

## Backends de banco de dados

| Motor | Status |
|---|---|
| SQLite | **Totalmente real**: conexão, consultas, CRUD, DDL, scripts, chaves estrangeiras, PRAGMA (`rusqlite` bundled, multiplataforma) |
| PostgreSQL | **Protocolo de rede real**: teste de conexão + consultas reais (driver `postgres` em Rust puro) |
| MySQL / MariaDB / Oracle / SQL Server / MongoDB / Redis / Snowflake / Dameng / ODBC | Config de conexão + schema de demo offline (UI/fluxo completos; camada de driver abstraída, extensível conforme `src/realdb.rs`) |

## Argumentos de linha de comando

| Argumento | Descrição |
|---|---|
| (nenhum) | Inicia a GUI |
| `--selftest` | Autoteste sem interface, imprime `[PASS]/[FAIL]` e retorna um código de saída (0 = tudo passa) |

## Variáveis de ambiente (opcional)

| Variável | Descrição |
|---|---|
| `NAVICAT_RS_THEME` | Tema na inicialização: `light`/`dark`/`midnight`/`forest`/`sand`/`graphite` |
| `NAVICAT_RS_DEMO` | `1` abre consultas e tabelas de exemplo; `all` abre todas as páginas de recursos (para capturas/aceitação) |
| `NAVICAT_RS_TAB` | Com demo, muda para a aba cujo título contém essa substring |

## Perguntas frequentes (FAQ)

**P: Preciso de conta ou licença do Navicat?** Não. O app abre direto na UI principal; sem avisos de login / ativação / teste.

**P: Onde os dados são armazenados?** Veja "Área de trabalho"; Windows `%APPDATA%\navicat-rs\navicat-rs.json`; o banco em si é o seu arquivo SQLite conectado.

**P: Por que o binário do macOS é compilado na CI?** Esta máquina é Windows; o binário universal do macOS é produzido pelo GitHub Actions `macos-latest` (veja `build.yml`).

**P: Um motor não-SQLite conecta de verdade?** SQLite é totalmente real e o PostgreSQL usa o protocolo de rede real; os demais são config + BD de demo offline, extensíveis conforme `src/realdb.rs`.

## Feedback (Issues)

Use o GitHub Issues: <https://github.com/shuying-code/navicat-rs/issues/new/choose>. Leia o **[guia de feedback](docs/FEEDBACK.md)** antes de enviar. Os mantenedores costumam tratar as issues uma vez por semana.

## Licença

[MIT](LICENSE)
