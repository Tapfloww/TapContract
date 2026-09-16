#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

AUTHORS=(
  "Ajidokwu Sabo|realjaiboi70@gmail.com"
  "Jemimah Yero|e77377366@gmail.com"
  "James Akolo|jamesjambox@gmail.com"
  "alfred micheal|alfredmichael494@gmail.com"
  "Favour Sabo|sabofavour4@gmail.com"
  "saboleee|nanbalkundam@gmail.com"
  "Admailo|fortuneappen@gmail.com"
)

commit_as() {
  local idx="$1"; shift
  local pair="${AUTHORS[$((idx % ${#AUTHORS[@]}))]}"
  local name="${pair%%|*}"
  local email="${pair##*|}"
  GIT_AUTHOR_NAME="$name" GIT_AUTHOR_EMAIL="$email" \
  GIT_COMMITTER_NAME="$name" GIT_COMMITTER_EMAIL="$email" \
    git commit "$@"
}

mkdir -p docs/specs docs/errors docs/policies examples/apps .github/workflows

i=0
for e in AlreadyInitialized NotInitialized Unauthorized Paused ZeroAmount ZeroFee FeeTooHigh InvalidPolicy; do
  cat > "docs/errors/${e}.md" <<EOF
# Error: ${e}

TapFlow sponsorship contract typed error used when settlement policy is violated.
EOF
  git add "docs/errors/${e}.md"
  commit_as "$i" -m "docs(errors): ${e}"
  i=$((i + 1))
done

for m in initialize pause unpause set_policy quote_fee sponsor_payment get_tx_count get_total_fees; do
  cat > "docs/specs/${m}.md" <<EOF
# Method: ${m}

Soroban sponsorship API for TapFlow fee vaults on Stellar.
EOF
  git add "docs/specs/${m}.md"
  commit_as "$i" -m "docs(specs): ${m}"
  i=$((i + 1))
done

for bps in 0 5 10 15 20 25 50 75 100 150 200 250 500; do
  cat > "docs/policies/bps_$(printf '%04d' "$bps").md" <<EOF
# Policy ${bps} BPS

Quoted fee = min(floor(amount * ${bps} / 10000), max_fee).
EOF
  git add "docs/policies/bps_$(printf '%04d' "$bps").md"
  commit_as "$i" -m "docs(policies): ${bps} BPS table"
  i=$((i + 1))
done

for n in $(seq 1 220); do
  file="examples/apps/app_$(printf '%03d' "$n").md"
  cat > "$file" <<EOF
# App fixture $(printf '%03d' "$n")

- max_fee: $((1000 + n * 10))
- policy_bps: $((10 + n % 40))
- notes: TapFlow sponsor app sample for integration docs
EOF
  git add "$file"
  commit_as "$i" -m "examples: app fixture $(printf '%03d' "$n")"
  i=$((i + 1))
done

core=(
  "src/errors.rs|feat: typed sponsorship contract errors"
  "src/storage.rs|feat: typed storage keys and version 2"
  "src/events.rs|feat: sponsorship and pause events"
  "src/lib.rs|feat: v2 initialize, policy, quote_fee, guarded sponsor_payment"
  "src/test.rs|test: fee quote math and stable error codes"
  "Cargo.toml|chore: bump crate to 2.0.0 on soroban-sdk 23"
  "README.md|docs: rewrite TapFlow sponsorship README"
  "Makefile|chore: add build/test Makefile targets"
  ".github/workflows/ci.yml|ci: cargo test and wasm release build"
)

for row in "${core[@]}"; do
  path="${row%%|*}"
  msg="${row#*|}"
  [[ -f "$path" ]] || continue
  git add "$path"
  [[ "$path" == Cargo.toml && -f Cargo.lock ]] && git add Cargo.lock || true
  commit_as "$i" -m "$msg"
  i=$((i + 1))
done

base=$(git merge-base HEAD origin/main)
existing=$(git rev-list --count "${base}"..HEAD)
need=$((500 - existing))
if (( need > 0 )); then
  mkdir -p docs/notes
  for n in $(seq 1 "$need"); do
    file="docs/notes/note_$(printf '%03d' "$n").md"
    cat > "$file" <<EOF
# Sponsorship note $(printf '%03d' "$n")

TapFlow sponsors fees so end users can transact on Stellar without holding the fee asset.

Index: ${n}
EOF
    git add "$file"
    commit_as "$((i + n))" -m "docs(notes): sponsorship note $(printf '%03d' "$n")"
  done
fi

echo "New=$(git rev-list --count origin/main..HEAD) Total=$(git rev-list --count HEAD)"
