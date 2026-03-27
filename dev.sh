#!/usr/bin/env bash
set -euo pipefail

# ══════════════════════════════════════════════════════════════════════════════
# dev.sh — Development script for chromata
#
# Usage: ./dev.sh <command> [flags]
#
# Commands: examples, test, build, lint, clean, fetch, generate, check, ci,
#           doctor, info, help
# ══════════════════════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# ── Colours ──────────────────────────────────────────────────────────────────

if [[ -t 1 ]]; then
    RED=$'\e[0;31m'
    GREEN=$'\e[0;32m'
    YELLOW=$'\e[0;33m'
    BLUE=$'\e[0;34m'
    MAGENTA=$'\e[0;35m'
    CYAN=$'\e[0;36m'
    BOLD=$'\e[1m'
    DIM=$'\e[2m'
    RESET=$'\e[0m'
else
    RED='' GREEN='' YELLOW='' BLUE='' MAGENTA='' CYAN='' BOLD='' DIM='' RESET=''
fi

# ── Helpers ──────────────────────────────────────────────────────────────────

log()  { echo -e "${CYAN}▸${RESET} $*"; }
ok()   { echo -e "${GREEN}✓${RESET} $*"; }
warn() { echo -e "${YELLOW}⚠${RESET} $*"; }
err()  { echo -e "${RED}✗${RESET} $*" >&2; }
hdr()  { echo -e "\n${BOLD}${BLUE}── $* ──${RESET}\n"; }

has_cmd() { command -v "$1" &>/dev/null; }

require_cmd() {
    if ! has_cmd "$1"; then
        err "$1 is not installed"
        if [[ -n "${2:-}" ]]; then
            echo -e "  ${DIM}install: $2${RESET}" >&2
        fi
        exit 1
    fi
}

# ── Examples ─────────────────────────────────────────────────────────────────

ALL_EXAMPLES=(list_all preview_ansi export_css find_theme ratatui_demo egui_gallery)

# Features required by specific examples
declare -A EXAMPLE_FEATURES=(
    [ratatui_demo]="ratatui-integration"
    [egui_gallery]="egui-integration"
)

cmd_examples() {
    local run_all=false
    local profile="release"
    local list=false
    local single=""

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --all)       run_all=true ;;
            --release)   profile="release" ;;
            --debug)     profile="debug" ;;
            --list)      list=true ;;
            --help|-h)   cmd_examples_help; return ;;
            -*)          err "unknown flag: $1"; cmd_examples_help; return 1 ;;
            *)           single="$1" ;;
        esac
        shift
    done

    if $list; then
        hdr "examples"
        for ex in "${ALL_EXAMPLES[@]}"; do
            local feature="${EXAMPLE_FEATURES[$ex]:-}"
            if [[ -n "$feature" ]]; then
                echo -e "  ${BOLD}$ex${RESET}  ${YELLOW}(--features $feature)${RESET}"
            else
                echo -e "  ${BOLD}$ex${RESET}"
            fi
        done
        return
    fi

    if [[ -n "$single" ]]; then
        run_example "$single" "$profile"
        return
    fi

    if $run_all; then
        hdr "all examples (${#ALL_EXAMPLES[@]})"
        local i=0
        for ex in "${ALL_EXAMPLES[@]}"; do
            i=$((i + 1))
            log "[$i/${#ALL_EXAMPLES[@]}]"
            run_example "$ex" "$profile"
        done
        ok "all ${#ALL_EXAMPLES[@]} examples complete"
        return
    fi

    cmd_examples_help
}

run_example() {
    local name="$1"
    local profile="${2:-release}"
    local cargo_args=(--"$profile" --example "$name")

    local feature="${EXAMPLE_FEATURES[$name]:-}"
    if [[ -n "$feature" ]]; then
        cargo_args+=(--features "$feature")
    fi

    hdr "$name${feature:+ (--features $feature)}"
    cargo run "${cargo_args[@]}"
    ok "$name"
}

cmd_examples_help() {
    cat <<'EOF'
Usage: ./dev.sh examples [name] [flags]

Run examples — all or a single one.

Flags:
  --all         run all examples
  --release     release build (default)
  --debug       debug build
  --list        list all examples

Examples:
  ./dev.sh examples list_all          # single example
  ./dev.sh examples --all             # run all examples
  ./dev.sh examples --list            # list examples
EOF
}

# ── test ─────────────────────────────────────────────────────────────────────

cmd_test() {
    local run_all=false
    local mode=""
    local filter=""
    local extra_args=()

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --all)        run_all=true ;;
            --release)    mode="release" ;;
            --filter)     shift; filter="$1" ;;
            --help|-h)    cmd_test_help; return ;;
            *)            extra_args+=("$1") ;;
        esac
        shift
    done

    local cargo_args=()
    [[ "$mode" == "release" ]] && cargo_args+=(--release)
    $run_all && cargo_args+=(--all-features)

    local test_args=(-- --test-threads=1)
    [[ -n "$filter" ]] && test_args+=("$filter")
    test_args+=("${extra_args[@]}")

    hdr "test chromata${mode:+ ($mode)}${run_all:+ (all features)}"
    log "cargo test ${cargo_args[*]} ${test_args[*]}"
    cargo test "${cargo_args[@]}" "${test_args[@]}"
    ok "tests passed"
}

cmd_test_help() {
    cat <<'EOF'
Usage: ./dev.sh test [flags]

Flags:
  --all               test with --all-features
  --release           release mode
  --filter <pattern>  filter test names
EOF
}

# ── build ────────────────────────────────────────────────────────────────────

cmd_build() {
    local profile=""
    local features=""
    local run_all=false
    local extra_args=()

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --release)   profile="release" ;;
            --features)  shift; features="$1" ;;
            --all)       run_all=true ;;
            --help|-h)   cmd_build_help; return ;;
            *)           extra_args+=("$1") ;;
        esac
        shift
    done

    local build_args=()
    [[ -n "$profile" ]]  && build_args+=(--release)
    [[ -n "$features" ]] && build_args+=(--features "$features")
    $run_all && build_args+=(--all-features)
    build_args+=("${extra_args[@]}")

    hdr "build chromata${profile:+ ($profile)}${run_all:+ (all features)}"
    log "cargo build ${build_args[*]}"
    cargo build "${build_args[@]}"
    ok "build complete"
}

cmd_build_help() {
    cat <<'EOF'
Usage: ./dev.sh build [flags]

Flags:
  --release       release profile
  --features <f>  comma-separated features
  --all           build with --all-features
EOF
}

# ── lint ─────────────────────────────────────────────────────────────────────

cmd_lint() {
    local fix=false
    local run_all=false

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --fix)     fix=true ;;
            --all)     run_all=true ;;
            --help|-h) cmd_lint_help; return ;;
        esac
        shift
    done

    local feature_args=()
    $run_all && feature_args+=(--all-features)

    hdr "lint chromata"

    if $fix; then
        log "cargo fmt"
        cargo fmt
        log "cargo clippy --fix --allow-dirty --all-targets ${feature_args[*]}"
        cargo clippy --fix --allow-dirty --all-targets "${feature_args[@]}"
    else
        log "cargo fmt --check"
        cargo fmt --check
        log "cargo clippy --all-targets ${feature_args[*]} -- -D warnings"
        cargo clippy --all-targets "${feature_args[@]}" -- -D warnings
    fi

    hdr "lint xtask"
    if $fix; then
        log "cargo clippy -p xtask --fix --allow-dirty"
        cargo clippy -p xtask --fix --allow-dirty
    else
        log "cargo clippy -p xtask -- -D warnings"
        cargo clippy -p xtask -- -D warnings
    fi

    hdr "docs"
    log "RUSTDOCFLAGS=\"-D warnings\" cargo doc --no-deps --all-features"
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

    ok "lint passed"
}

cmd_lint_help() {
    cat <<'EOF'
Usage: ./dev.sh lint [flags]

Runs fmt + clippy (library + xtask) + doc build.

Flags:
  --fix   apply clippy fixes and format code
  --all   lint with --all-features
EOF
}

# ── clean ────────────────────────────────────────────────────────────────────

cmd_clean() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --help|-h) cmd_clean_help; return ;;
        esac
        shift
    done

    hdr "clean chromata"
    log "cargo clean"
    cargo clean
    ok "clean complete"
}

cmd_clean_help() {
    cat <<'EOF'
Usage: ./dev.sh clean

Removes all build artifacts (target/).
For data/generated file cleanup, use: ./dev.sh xtask clean
EOF
}

# ── xtask ────────────────────────────────────────────────────────────────────

cmd_xtask() {
    local subcmd="${1:-help}"
    shift 2>/dev/null || true
    case "$subcmd" in
        fetch)    hdr "fetch"; cargo xtask fetch "${1:-all}"; ok "fetch complete" ;;
        generate) hdr "generate"; cargo xtask generate "${1:-all}"; ok "generate complete" ;;
        check)    hdr "freshness check"; cargo xtask check; ok "all files up to date" ;;
        clean)    hdr "xtask clean"; cargo xtask clean "$@"; ok "clean complete" ;;
        ci)       hdr "xtask ci"; cargo xtask ci; ok "xtask ci complete" ;;
        all)      hdr "xtask all"; cargo xtask fetch all && cargo xtask generate all; ok "fetch + generate complete" ;;
        help|*)
            echo -e "${BOLD}Usage:${RESET} ./dev.sh xtask <subcommand> [args]"
            echo ""
            echo -e "${BOLD}Subcommands:${RESET}"
            echo -e "  ${CYAN}fetch${RESET} [collection|all]       Download upstream theme data"
            echo -e "  ${CYAN}generate${RESET} [collection|all]    Generate Rust source from data/"
            echo -e "  ${CYAN}check${RESET}                        Verify generated files are up-to-date"
            echo -e "  ${CYAN}clean${RESET} [--cache|--generated]  Remove fetched data / generated files"
            echo -e "  ${CYAN}ci${RESET}                           Fetch + generate + check + clippy + test"
            echo -e "  ${CYAN}all${RESET}                          Fetch + generate all"
            ;;
    esac
}

# ── check (feature isolation) ────────────────────────────────────────────────

cmd_check() {
    hdr "feature isolation"
    local features=(popular base16 base24 vim emacs)
    for f in "${features[@]}"; do
        log "checking --features $f..."
        cargo check --no-default-features --features "$f"
        ok "$f"
    done

    log "checking no_std (no features)..."
    cargo check --no-default-features
    ok "no_std"

    ok "all feature combinations compile"
}

# ── snapshots ────────────────────────────────────────────────────────────────

cmd_snapshots() {
    local subcmd="${1:-review}"
    shift 2>/dev/null || true
    case "$subcmd" in
        review)
            hdr "snapshot review"
            require_cmd cargo-insta "cargo install cargo-insta"
            cargo insta review
            ;;
        update)
            hdr "snapshot update"
            require_cmd cargo-insta "cargo install cargo-insta"
            log "running tests to generate new snapshots..."
            cargo test --all-features -- --test-threads=1 || true
            cargo insta accept
            ok "snapshots updated"
            ;;
        test)
            hdr "snapshot test"
            INSTA_UPDATE=no cargo test --test snapshots --all-features -- --test-threads=1
            ok "snapshot tests passed"
            ;;
        help|*)
            echo -e "${BOLD}Usage:${RESET} ./dev.sh snapshots <subcommand>"
            echo ""
            echo -e "${BOLD}Subcommands:${RESET}"
            echo -e "  ${CYAN}review${RESET}   Interactively review pending snapshots (default)"
            echo -e "  ${CYAN}update${RESET}   Regenerate and accept all snapshots"
            echo -e "  ${CYAN}test${RESET}     Run snapshot tests without updating"
            ;;
    esac
}

# ── ci ───────────────────────────────────────────────────────────────────────

cmd_ci() {
    hdr "full CI (local)"

    log "step 1/5: lint"
    cmd_lint --all

    log "step 2/5: tests"
    cmd_test --all

    log "step 3/5: feature isolation"
    cmd_check

    log "step 4/5: freshness"
    cargo xtask check
    ok "generated files up to date"

    log "step 5/5: examples"
    cmd_examples --all

    echo ""
    ok "all CI checks passed"
}

# ── doctor ───────────────────────────────────────────────────────────────────

TOOL_REGISTRY=(
    "rustc:rustc:curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh:required"
    "cargo:cargo:(installed with rustc):required"
    "clippy:cargo-clippy:rustup component add clippy:required"
    "rustfmt:rustfmt:rustup component add rustfmt:required"
    "cargo-insta:cargo-insta:cargo install cargo-insta:recommended"
    "cargo-semver-checks:cargo-semver-checks:cargo install cargo-semver-checks:optional"
)

check_tool() {
    local name="$1" cmd="$2" install="$3" category="$4"
    if has_cmd "$cmd"; then
        echo -e "  ${GREEN}✓${RESET} $name"
        return 0
    else
        local tag
        case "$category" in
            required)    tag="${RED}REQUIRED${RESET}" ;;
            recommended) tag="${YELLOW}recommended${RESET}" ;;
            optional)    tag="${DIM}optional${RESET}" ;;
        esac
        echo -e "  ${RED}✗${RESET} $name  [$tag]"
        echo -e "    ${DIM}→ $install${RESET}"
        return 1
    fi
}

cmd_doctor() {
    hdr "doctor — checking prerequisites"

    local missing_required=0

    for entry in "${TOOL_REGISTRY[@]}"; do
        IFS=':' read -r name cmd install category <<< "$entry"
        if ! check_tool "$name" "$cmd" "$install" "$category"; then
            case "$category" in
                required) missing_required=$((missing_required + 1)) ;;
            esac
        fi
    done

    echo ""
    if [[ $missing_required -eq 0 ]]; then
        ok "all required tools installed"
    else
        err "$missing_required required tool(s) missing"
        exit 1
    fi
}

# ── info ─────────────────────────────────────────────────────────────────────

cmd_info() {
    hdr "system info"

    echo -e "  ${BOLD}project${RESET}    chromata"
    echo -e "  ${BOLD}version${RESET}    $(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)"
    echo -e "  ${BOLD}directory${RESET}  $(pwd)"

    echo ""
    if has_cmd rustc; then
        echo -e "  ${BOLD}rustc${RESET}      $(rustc --version)"
    fi
    if has_cmd cargo; then
        echo -e "  ${BOLD}cargo${RESET}      $(cargo --version)"
    fi

    hdr "feature flags"
    echo "  popular    (default) 49 curated themes"
    echo "  base16     305 tinted-theming base16 themes"
    echo "  base24     184 tinted-theming base24 themes"
    echo "  vim        464 vim colorschemes"
    echo "  emacs      102 emacs themes"
    echo "  all        all theme families combined"
    echo ""
    echo "  ratatui-integration    From<Color> for ratatui types"
    echo "  egui-integration       From<Color> for egui types"
    echo "  crossterm-integration  From<Color> for crossterm types"
    echo "  iced-integration       From<Color> for iced types"
    echo "  serde-support          Serialize/deserialize themes"

    hdr "examples"
    echo -e "  ${BOLD}${#ALL_EXAMPLES[@]}${RESET} examples"
    for ex in "${ALL_EXAMPLES[@]}"; do
        local feature="${EXAMPLE_FEATURES[$ex]:-}"
        if [[ -n "$feature" ]]; then
            echo -e "    ${DIM}$ex${RESET}  ${YELLOW}(--features $feature)${RESET}"
        else
            echo -e "    ${DIM}$ex${RESET}"
        fi
    done
}

# ── help ─────────────────────────────────────────────────────────────────────

cmd_help() {
    cat <<EOF
${BOLD}dev.sh${RESET} — development script for ${BOLD}chromata${RESET}

${BOLD}Usage:${RESET} ./dev.sh <command> [flags]

${BOLD}Commands:${RESET}
  ${CYAN}doctor${RESET}     check prerequisites
  ${CYAN}examples${RESET}   run examples (all or single)
               --all  --release  --debug  --list
  ${CYAN}test${RESET}       run tests
               --all  --release  --filter <pattern>
  ${CYAN}build${RESET}      build library
               --all  --release  --features <f>
  ${CYAN}lint${RESET}       fmt + clippy (library + xtask) + doc build
               --fix  --all
  ${CYAN}clean${RESET}      clean build artifacts

${BOLD}Xtask:${RESET}
  ${CYAN}xtask${RESET}      run xtask subcommands
               fetch, generate, check, clean, ci
  ${CYAN}check${RESET}      test each feature flag compiles independently
  ${CYAN}ci${RESET}         run full local CI pipeline
  ${CYAN}snapshots${RESET}  review, update, or test insta snapshots

${BOLD}Other:${RESET}
  ${CYAN}info${RESET}       show project info, features, examples
  ${CYAN}help${RESET}       this message

${BOLD}Examples:${RESET}
  ./dev.sh doctor                    # check prerequisites
  ./dev.sh examples list_all         # run single example
  ./dev.sh examples --all            # run all examples
  ./dev.sh test --all                # test with all features
  ./dev.sh lint --fix --all          # auto-fix all lint issues
  ./dev.sh ci                        # full CI pipeline
  ./dev.sh xtask fetch all           # fetch all upstream data
  ./dev.sh xtask generate base16     # regenerate base16 themes
EOF
}

# ══════════════════════════════════════════════════════════════════════════════
# Dispatch
# ══════════════════════════════════════════════════════════════════════════════

if [[ $# -lt 1 ]]; then
    cmd_help
    exit 0
fi

command="$1"; shift

case "$command" in
    examples)  cmd_examples "$@" ;;
    test)      cmd_test "$@" ;;
    build)     cmd_build "$@" ;;
    lint)      cmd_lint "$@" ;;
    clean)     cmd_clean "$@" ;;
    xtask)     cmd_xtask "$@" ;;
    check)     cmd_check "$@" ;;
    ci)        cmd_ci "$@" ;;
    snapshots) cmd_snapshots "$@" ;;
    doctor)    cmd_doctor ;;
    info)      cmd_info ;;
    help|-h|--help) cmd_help ;;
    *)
        err "unknown command: $command"
        cmd_help
        exit 1
        ;;
esac
