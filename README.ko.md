<p align="center">🌐 <b>언어</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# navicat-rs

**Rust + egui/eframe** 로 만든 **크로스 플랫폼 단일 파일** 데이터베이스 관리 데스크톱 클라이언트입니다. UI와 워크플로는 Navicat Premium 18 을 참고했습니다.
**모든 기능을 무료로 개방합니다. 로그인, 체험, 라이선스/권한 제한이 없습니다.**

<p>
  <img alt="CI" src="https://github.com/shuying-code/navicat-rs/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/navicat-rs/actions/workflows/build.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.75%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/navicat-rs?display_name=tag">
</p>

> 본 프로젝트는 **로컬 우선** 독립 구현이며 클라우드 연결, 로그인, 구독, 체험, 텔레메트리가 없습니다. **모든 기능을 누구나 무조건 사용할 수 있습니다.**

![환영](assets/screenshot-welcome.png)

---

## 왜 "무제한"인가

Navicat 의 유료/로그인 제약은 주로 **클라우드/구독** 기능(클라우드 동기화, 팀 협업, 클라우드 자동화 등)에 대한 것입니다. 이 도구는 **클라우드에 의존하지 않는 로컬** 데이터베이스 관리만 구현하므로 계정도, 결제도, 할당량도 필요 없습니다. 설계상 로그인/구독/라이선스 제한을 제거하여 모든 로컬 기능을 무조건 사용할 수 있습니다.

---

## 기능

- **테이블**: 페이징 탐색, 필터, 인라인 편집, 행 추가/삭제, 정렬(헤더 클릭), 내보내기.
- **SQL 쿼리**: 다중 문 실행, `Ctrl+Enter`, 다중 그리드 결과, 소요 시간/행 수, Explain 실행 계획 트리, SQL 정렬, SQL 개요/리팩터 힌트, 쿼리 기록.
- **설계 및 모델링**: 테이블 설계(열 편집 → DDL), ER 다이어그램, 쿼리 빌더, 모델 설계(개념/논리/물리 · 리버스 엔지니어링 · 비교 · 동기화).
- **데이터 이동**: 데이터 전송(열 매핑 + WHERE), 데이터/구조 동기화, 가져오기 마법사(CSV/JSON/SQL), 내보내기 마법사(CSV/JSON/SQL/HTML), 데이터 펌프.
- **운영 및 확장**: 백업/복원(인메모리 DB `VACUUM INTO` 포함), 서버 모니터링, 프로파일링, 스키마 분석, 데이터 생성, 마스킹, 차트, 데이터 사전, 리포트.
- **객체 및 검색**: 객체 관리(인덱스/트리거/뷰), 데이터베이스 내 검색(전체 테이블 열 값 검색), 객체 트리 확대/축소 및 그룹 접기.
- **기타**: AI 어시스턴트, 자동화/예약 작업, 코드 스니펫, 사용자 및 권한(모두 개방), 설정(멀티 테마/트리 확대/자동 커밋/삭제 확인).
- **멀티 테마**: 라이트/다크/미드나이트/포레스트/샌드/그래파이트. 모든 테마에서 본문과 버튼 글자가 고대비로 읽기 쉬움.

## GUI

- **순수 Rust + egui/eframe**: 단일 파일 exe, WebView 불필요, 더블클릭 실행. CJK 폰트 내장으로 글자 깨짐 없음.
- **3구역 레이아웃**: 메뉴바 / 툴바 / 탭바 / 좌측 연결 및 객체 트리 / 우측 기능 페이지 / 상태 표시줄.
- **객체 트리**: 연결 → 테이블/뷰/백업/모델/사용자. 확대/축소 가능(`− 100% ＋`, `Ctrl+휠`, 설정 슬라이더), 그룹 접기 가능.
- **멀티 탭**: 테이블, 쿼리, 설계, ER, 빌더 및 각 도구 페이지를 병렬로 엽니다.
- **멀티 테마**: `보기 → 테마` 또는 `설정 → 테마` 로 전환하며 선택이 저장됩니다.

## 지원 플랫폼

| 플랫폼 | 상태 | 설명 |
|---|---|---|
| Windows x64 | ✅ 빌드 검증됨 | MSVC 툴체인 권장; Release 에 빌드된 exe |
| macOS(Intel / Apple Silicon) | ✅ CI 빌드(universal) | `build.yml` 이 유니버설 바이너리 생성 |
| Linux(X11 / Wayland) | ✅ CI 빌드 | `build.yml` 이 x64 아카이브 생성(시스템 GUI 의존 필요) |

## 빌드

Rust 1.75+ 필요(stable 권장).

```bash
cargo build --release
```

### Windows(MSVC)

```powershell
# Visual Studio Build Tools(C++ 및 Windows SDK) 필요
cargo build --release
```

### macOS

```bash
xcode-select --install   # 최초 필요할 수 있음
cargo build --release
```

### Linux(Debian/Ubuntu)

```bash
sudo apt-get install -y build-essential pkg-config \
  libxkbcommon-dev libgtk-3-dev libgl1-mesa-dev \
  libx11-dev libxi-dev libxrandr-dev libxcursor-dev libxinerama-dev \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
cargo build --release
```

## 사전 빌드 / 크로스 플랫폼 산출물

- **Releases**: <https://github.com/shuying-code/navicat-rs/releases> (Windows x64 exe/zip, macOS universal tar.gz, Linux x64 tar.gz; `build.yml` 이 생성).
- **CI 매트릭스**: `.github/workflows/ci.yml` 이 `ubuntu-latest / macos-latest / windows-latest` 에서 빌드하고 `--selftest` 실행.
- **컨테이너 이미지(GHCR)**:

```bash
docker pull ghcr.io/shuying-code/navicat-rs:latest
docker run --rm ghcr.io/shuying-code/navicat-rs:latest --selftest
```

## 사용법

```bash
navicat-rs              # GUI 실행
navicat-rs --selftest   # 헤드리스 자체 테스트(SQLite 엔진 + 핵심 기능, 22 항목)
```

최초 실행 시 "샘플 데이터베이스(SQLite)"(테이블 4개 + 뷰 1개 + 데모 데이터)를 자동 생성합니다. 객체 트리에서 테이블을 더블클릭해 탐색하고, 상단 툴바에서 쿼리/테이블 설계/ER/쿼리 빌더/전송/가져오기/내보내기/백업/모니터/차트/프로파일링/AI 등 모든 페이지를 엽니다.

## 워크스페이스(로컬 설정)

설정과 상태(연결, 기록, 즐겨찾기, 테마, 설정)는 각 플랫폼의 사용자 설정 디렉터리에 저장됩니다:

| 플랫폼 | 경로 |
|---|---|
| Windows | `%APPDATA%\navicat-rs\navicat-rs.json` |
| macOS | `~/Library/Application Support/navicat-rs/navicat-rs.json` |
| Linux | `$XDG_CONFIG_HOME/navicat-rs/navicat-rs.json` 또는 `~/.config/navicat-rs/navicat-rs.json` |

## 검증

- `--selftest` 가 엔진과 핵심 기능 **22 항목 전부 통과**.
- CI 가 3개 플랫폼(Windows / macOS / Ubuntu)에서 빌드 + 자체 테스트 **모두 success**.
- 6개 테마를 스크린샷으로 본문/굵은 제목/헤더/버튼 글자 가독성 확인.

## 데이터베이스 백엔드

| 엔진 | 상태 |
|---|---|
| SQLite | **완전 실제**: 연결, 쿼리, CRUD, DDL, 스크립트, 외래 키, PRAGMA(`rusqlite` bundled, 크로스 플랫폼) |
| PostgreSQL | **실제 네트워크 프로토콜**: 연결 테스트 + 실제 쿼리(순수 Rust `postgres` 드라이버) |
| MySQL / MariaDB / Oracle / SQL Server / MongoDB / Redis / Snowflake / Dameng / ODBC | 연결 구성 + 오프라인 데모 schema(UI/워크플로 완비; 드라이버 계층 추상화, `src/realdb.rs` 패턴으로 추가 가능) |

## 명령줄 인자

| 인자 | 설명 |
|---|---|
| (없음) | GUI 실행 |
| `--selftest` | 헤드리스 자체 테스트 실행, `[PASS]/[FAIL]` 출력 후 종료 코드 반환(0=전부 통과) |

## 환경 변수(선택)

| 변수 | 설명 |
|---|---|
| `NAVICAT_RS_THEME` | 시작 시 테마: `light`/`dark`/`midnight`/`forest`/`sand`/`graphite` |
| `NAVICAT_RS_DEMO` | `1` 은 샘플 쿼리와 테이블 열기; `all` 은 모든 기능 페이지 열기(스크린샷/검증용) |
| `NAVICAT_RS_TAB` | demo 와 함께 사용, 제목에 해당 부분 문자열이 포함된 탭으로 전환 |

## 자주 묻는 질문(FAQ)

**Q: Navicat 계정이나 라이선스가 필요한가요?** 아닙니다. 실행하면 바로 메인 UI 로 진입하며 로그인/활성화/체험 안내가 없습니다.

**Q: 데이터는 어디에 저장되나요?** "워크스페이스" 참고. Windows 는 `%APPDATA%\navicat-rs\navicat-rs.json`, 데이터베이스 자체는 연결한 SQLite 파일에 따릅니다.

**Q: macOS 바이너리는 왜 CI 에서 빌드하나요?** 이 머신은 Windows 이므로 macOS 유니버설 바이너리는 GitHub Actions `macos-latest` 가 생성합니다(`build.yml` 참고).

**Q: 비 SQLite 엔진은 실제로 연결되나요?** SQLite 는 완전 실제, PostgreSQL 은 실제 네트워크 프로토콜입니다. 나머지는 연결 구성 + 오프라인 데모 DB 이며 `src/realdb.rs` 패턴으로 확장할 수 있습니다.

## 피드백(Issues)

GitHub Issues 를 사용하세요: <https://github.com/shuying-code/navicat-rs/issues/new/choose>. 제출 전 **[피드백 가이드](docs/FEEDBACK.md)** 를 읽어 주세요. 관리자는 보통 주 1회 일괄 처리합니다.

## 라이선스

[MIT](LICENSE)
