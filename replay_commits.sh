#!/usr/bin/env bash
# replay_commits.sh
# Creates the full git history for nix-snake from scratch.
# Run from the repo root after `cargo new nix-snake --name nix-snake`.
#
# Usage:
#   bash replay_commits.sh
#
# Each step copies the staged main.rs into src/ and makes a commit,
# so you end up with a real git log you can `git log --oneline` through.

set -e

DOCS="$(dirname "$0")/docs"

commit_stage() {
    local num="$1"
    local msg="$2"
    local src="$DOCS/$num/main.rs"

    if [ -f "$src" ]; then
        cp "$src" src/main.rs
        git add src/main.rs Cargo.toml README.md 2>/dev/null || true
        git commit -m "$msg"
        echo "✓ $msg"
    else
        echo "⚠ skipping $num (no main.rs found)"
    fi
}

# Make sure Cargo.toml and README are staged in the first commit
git add Cargo.toml README.md 2>/dev/null || true

commit_stage "01_init"             "chore: init project with Cargo.toml and empty main"
commit_stage "02_direction"        "feat: add Direction type alias and cardinal constants"
commit_stage "03_snake_struct"     "feat: add Snake struct with VecDeque body"
commit_stage "04_snake_movement"   "feat: add snake movement with wrapping and collision detection"
commit_stage "05_food"             "feat: add Food struct with random spawning"
commit_stage "06_mode_autopilot"   "feat: add Mode enum and greedy autopilot"
commit_stage "07_terminal_setup"   "feat: add terminal setup and teardown"
commit_stage "08_draw_game"        "feat: add draw_game with queued rendering"
commit_stage "09_input_collision"  "feat: add input handling and collision flash"

# Final commit uses the real src/main.rs (already in place)
git add src/main.rs
git commit -m "feat: add game loop, food eating, and panic hook"

echo ""
echo "Done. Run: git log --oneline"
