#!/bin/bash
set -euo pipefail

# ─── Server configuration ─────────────────────────────────────────────────────
SERVER_USER="root"
SERVER_HOST="192.168.1.101"
SERVER_PORT="22"
REMOTE_DIR="/root/portfolio-ssr"
APP_NAME="portfolio-ssr"
# ──────────────────────────────────────────────────────────────────────────────

SSH="ssh -p ${SERVER_PORT} -o ConnectTimeout=10"
SCP="scp -P ${SERVER_PORT}"
REMOTE="${SERVER_USER}@${SERVER_HOST}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
DIM='\033[2m'
NC='\033[0m'

# ─── Logging helpers ──────────────────────────────────────────────────────────
STEP=0
step()  {
  STEP=$((STEP + 1))
  echo -e "\n${BLUE}[${STEP}]${NC} ${WHITE}${1}${NC}"
}
ok()    { echo -e "    ${GREEN}✔${NC}  ${1}"; }
warn()  { echo -e "    ${YELLOW}⚠${NC}  ${1}"; }
info()  { echo -e "    ${DIM}${1}${NC}"; }
fail()  { echo -e "\n    ${RED}✖${NC}  ${1}\n"; exit 1; }

# Timer helpers
timer_start() { _TIMER_START=$(date +%s); }
timer_end()   {
  local elapsed=$(( $(date +%s) - _TIMER_START ))
  echo -e "    ${DIM}Done in ${elapsed}s${NC}"
}

# ─── Usage ───────────────────────────────────────────────────────────────────
usage() {
  echo -e "${WHITE}Usage:${NC} ./deploy.sh [options]\n"
  echo "  --build          Incremental build  (Cargo cache preserved)  ~2-5 min"
  echo "  --rebuild        Full rebuild        (no cache)              ~10-15 min"
  echo "  --fix-assets     Fix CRLF on SVGs and re-compress with brotli"
  echo "  --push-env       Upload local .env to the server"
  echo "  --down           Stop all containers on the server"
  echo "  --status         Show container status"
  echo "  --logs           Stream live logs from the server"
  echo "  -h, --help       Show this help"
  echo ""
  echo -e "${DIM}First deploy:   ./deploy.sh --push-env --rebuild${NC}"
  echo -e "${DIM}Everyday:       git push && ./deploy.sh --build${NC}"
}

# ─── Argument parsing ─────────────────────────────────────────────────────────
[[ $# -eq 0 ]] && { usage; exit 0; }

OPT_BUILD=false
OPT_REBUILD=false
OPT_FIX_ASSETS=false
OPT_PUSH_ENV=false
OPT_DOWN=false
OPT_STATUS=false
OPT_LOGS=false


while [[ $# -gt 0 ]]; do
  case $1 in
    --build)       OPT_BUILD=true; shift ;;
    --rebuild)     OPT_BUILD=true; OPT_REBUILD=true; shift ;;
    --fix-assets)  OPT_FIX_ASSETS=true; shift ;;
    --push-env)    OPT_PUSH_ENV=true; shift ;;
    --down)        OPT_DOWN=true; shift ;;
    --status)      OPT_STATUS=true; shift ;;
    --logs)        OPT_LOGS=true; shift ;;
    -h|--help)     usage; exit 0 ;;
    *) echo -e "${RED}Unknown option: $1${NC}"; usage; exit 1 ;;
  esac
done

# ─── Header ──────────────────────────────────────────────────────────────────
echo -e "\n${WHITE}══════════════════════════════════════════${NC}"
echo -e "${WHITE}  ${APP_NAME} — Remote Deploy${NC}"
echo -e "${DIM}  ${REMOTE}:${SERVER_PORT} → ${REMOTE_DIR}${NC}"
echo -e "${WHITE}══════════════════════════════════════════${NC}"

# ─── SSH connectivity check ───────────────────────────────────────────────────
step "Checking SSH connectivity"
if ! $SSH "$REMOTE" "exit 0" 2>/dev/null; then
  fail "Cannot reach ${REMOTE} on port ${SERVER_PORT}.\n      Make sure your SSH key is installed: ssh-copy-id -p ${SERVER_PORT} ${REMOTE}"
fi
ok "${REMOTE} reachable"

# ─── Status ──────────────────────────────────────────────────────────────────
if [[ "$OPT_STATUS" == true ]]; then
  step "Container status"
  $SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose ps"
  exit 0
fi

# ─── Down ────────────────────────────────────────────────────────────────────
if [[ "$OPT_DOWN" == true ]]; then
  step "Stopping containers"
  $SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose down"
  ok "All containers stopped"
  exit 0
fi

# ─── Push .env ───────────────────────────────────────────────────────────────
if [[ "$OPT_PUSH_ENV" == true ]]; then
  step "Uploading .env"
  [[ ! -f ".env" ]] && fail "No local .env file found."
  $SSH "$REMOTE" "mkdir -p ${REMOTE_DIR}"
  $SCP ".env" "${REMOTE}:${REMOTE_DIR}/.env"
  ok ".env uploaded to server"
fi

# ─── Fix assets ──────────────────────────────────────────────────────────────
if [[ "$OPT_FIX_ASSETS" == true ]]; then
  step "Fixing assets (CRLF → LF + brotli)"
  $SSH "$REMOTE" "cd ${REMOTE_DIR} && bash -s" << 'EOF'
    fixed=0
    while IFS= read -r f; do
      if file "$f" | grep -q CRLF; then
        sed -i 's/\r//' "$f"
        echo "      fixed CRLF: $f"
        fixed=$((fixed+1))
      fi
    done < <(find public/ -type f -name "*.svg")
    echo "      ${fixed} SVG(s) fixed"
    command -v brotli &>/dev/null || apt-get install -y brotli -qq
    count=0
    while IFS= read -r f; do
      brotli -q 11 -k -f "$f"
      count=$((count+1))
    done < <(find public/ -type f \( -name "*.svg" -o -name "*.webp" -o -name "*.png" -o -name "*.jpg" -o -name "*.js" \))
    echo "      ${count} file(s) compressed with brotli"
EOF
  ok "Assets ready"
fi

# ─── Git pull ────────────────────────────────────────────────────────────────
step "Syncing files to server"
timer_start
$SSH "$REMOTE" "mkdir -p ${REMOTE_DIR}"
rsync -az --delete \
  --exclude='.git/' \
  --exclude='target/' \
  --exclude='node_modules/' \
  --exclude='.env' \
  -e "ssh -p ${SERVER_PORT}" \
  ./ "${REMOTE}:${REMOTE_DIR}/"
timer_end
ok "Files synced"

# ─── Build ───────────────────────────────────────────────────────────────────
if [[ "$OPT_BUILD" == true ]]; then
  if [[ "$OPT_REBUILD" == true ]]; then
    step "Building images (no cache)"
    info "This recompiles everything from scratch. Estimated ~10-15 min."
    timer_start
    $SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose build --no-cache"
  else
    step "Building images (incremental)"
    info "Only changed layers will be rebuilt. Estimated ~2-5 min."
    timer_start
    $SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose build"
  fi
  timer_end
  ok "Build complete"
fi

# ─── Deploy ──────────────────────────────────────────────────────────────────
step "Starting services"
$SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose up -d"
ok "Containers started"

# ─── Health check ────────────────────────────────────────────────────────────
step "Waiting for health check"
info "Polling every 5s (max 50s)..."
for i in {1..10}; do
  STATUS=$($SSH "$REMOTE" "docker inspect --format='{{.State.Health.Status}}' portfolio-ssr 2>/dev/null" || echo "waiting")
  case "$STATUS" in
    healthy)
      ok "portfolio-ssr is healthy"
      break
      ;;
    unhealthy)
      fail "portfolio-ssr reported unhealthy. Run ./deploy.sh --logs to investigate."
      ;;
    *)
      info "[$i/10] Status: ${STATUS}"
      sleep 5
      ;;
  esac
done

# ─── Summary ─────────────────────────────────────────────────────────────────
echo -e "\n${WHITE}══════════════════════════════════════════${NC}"
echo -e "${GREEN}  ✔ Deployment successful${NC}"
echo -e "${WHITE}══════════════════════════════════════════${NC}"
echo -e "  ${DIM}Container status : ./deploy.sh --status${NC}"
echo -e "  ${DIM}Live logs        : ./deploy.sh --logs${NC}"

# ─── Discord notification ─────────────────────────────────────────────────────
DISCORD_URL=$($SSH "$REMOTE" "grep -E '^DISCORD_WEBHOOK_URL=' ${REMOTE_DIR}/.env 2>/dev/null | cut -d= -f2-" || true)
if [[ -n "$DISCORD_URL" ]]; then
  $SSH "$REMOTE" "curl -sf -X POST '${DISCORD_URL}' \
    -H 'Content-Type: application/json' \
    -d '{\"content\":\"**${APP_NAME}** deployed successfully \u1F680\"}' > /dev/null" \
    && echo -e "  ${DIM}Discord notified${NC}"
fi

echo ""

# ─── Logs ────────────────────────────────────────────────────────────────────
if [[ "$OPT_LOGS" == true ]]; then
  echo -e "${DIM}Streaming logs — Ctrl+C to exit${NC}\n"
  $SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose logs -f"
fi