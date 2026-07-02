#!/bin/bash
set -euo pipefail

# --- Configuration ----------------------------------------------------------
SERVER_USER="root"
SERVER_HOST="192.168.1.101"
SERVER_PORT="22"
REMOTE_DIR="/root/portfolio-ssr"
APP_NAME="portfolio-ssr"

REGISTRY_HOST="distribution.arsac-baptiste.dev"
IMAGE_NAME="${REGISTRY_HOST}/${APP_NAME}"

COMPOSE_FILE="docker-compose.yaml"
# -----------------------------------------------------------------------------

SSH="ssh -p ${SERVER_PORT} -o ConnectTimeout=10"
SCP="scp -P ${SERVER_PORT}"
REMOTE="${SERVER_USER}@${SERVER_HOST}"

GIT_TAG="$(git rev-parse --short HEAD 2>/dev/null || date +%Y%m%d%H%M%S)"

# --- Logging helpers ---------------------------------------------------------
STEP=0
step()  {
  STEP=$((STEP + 1))
  echo ""
  echo "[${STEP}] ${1}"
}
ok()    { echo "    OK    ${1}"; }
warn()  { echo "    WARN  ${1}"; }
info()  { echo "    ${1}"; }
fail()  { echo ""; echo "    FAIL  ${1}"; echo ""; exit 1; }

# Timer helpers
timer_start() { _TIMER_START=$(date +%s); }
timer_end()   {
  local elapsed=$(( $(date +%s) - _TIMER_START ))
  echo "    Done in ${elapsed}s"
}

# --- Usage --------------------------------------------------------------------
usage() {
  echo "Usage: ./deploy.sh [options]"
  echo ""
  echo "  --build          Incremental local build (Cargo cache kept)   ~2-5 min"
  echo "  --rebuild        Full local build (no cache)                  ~10-15 min"
  echo "  --fix-assets     Fix CRLF on SVGs and recompress with brotli (local)"
  echo "  --push-env       Upload local .env to the server (LXC100)"
  echo "  --down           Stop all containers on LXC100"
  echo "  --status         Show container status"
  echo "  --logs           Stream live logs from LXC100"
  echo "  -h, --help       Show this help"
  echo ""
  echo "First deploy:  ./deploy.sh --push-env --rebuild"
  echo "Everyday:      git pull && ./deploy.sh --build"
}

# --- Argument parsing -----------------------------------------------------------
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
    *) echo "Unknown option: $1"; usage; exit 1 ;;
  esac
done

# --- Header --------------------------------------------------------------------
echo ""
echo "=============================================="
echo "  ${APP_NAME} - deploy (local build -> ${REGISTRY_HOST} -> LXC100)"
echo "  Image  : ${IMAGE_NAME}:${GIT_TAG}"
echo "  Target : ${REMOTE}:${SERVER_PORT} -> ${REMOTE_DIR}"
echo "=============================================="

# --- Docker local check ---------------------------------------------------------
step "Checking local Docker"
command -v docker &>/dev/null || fail "Docker is not installed on this machine."
docker info &>/dev/null || fail "Docker daemon is not reachable (running? permissions?)."
ok "Local Docker is operational"

# --- SSH connectivity check (LXC100) --------------------------------------------
step "Checking SSH connectivity to LXC100"
if ! $SSH "$REMOTE" "exit 0" 2>/dev/null; then
  fail "Cannot reach ${REMOTE} on port ${SERVER_PORT}. Check your SSH key: ssh-copy-id -p ${SERVER_PORT} ${REMOTE}"
fi
ok "${REMOTE} reachable"

# --- Status -----------------------------------------------------------------------
if [[ "$OPT_STATUS" == true ]]; then
  step "Container status (LXC100)"
  $SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose ps"
  exit 0
fi

# --- Down -------------------------------------------------------------------------
if [[ "$OPT_DOWN" == true ]]; then
  step "Stopping containers (LXC100)"
  $SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose down"
  ok "All containers stopped"
  exit 0
fi

# --- Push .env ----------------------------------------------------------------------
if [[ "$OPT_PUSH_ENV" == true ]]; then
  step "Uploading .env to LXC100"
  [[ ! -f ".env" ]] && fail "No local .env file found."
  $SSH "$REMOTE" "mkdir -p ${REMOTE_DIR}"
  $SCP ".env" "${REMOTE}:${REMOTE_DIR}/.env"
  ok ".env uploaded to server"
fi

# --- Fix assets (local, before build) -----------------------------------------------
if [[ "$OPT_FIX_ASSETS" == true ]]; then
  step "Fixing assets locally (CRLF -> LF + brotli)"
  command -v brotli &>/dev/null || fail "brotli is not installed locally (e.g. apt/brew install brotli)."

  fixed=0
  while IFS= read -r f; do
    if file "$f" | grep -q CRLF; then
      sed -i 's/\r//' "$f"
      echo "      fixed CRLF: $f"
      fixed=$((fixed+1))
    fi
  done < <(find public/ -type f -name "*.svg")
  ok "${fixed} SVG(s) fixed"

  count=0
  while IFS= read -r f; do
    brotli -q 11 -k -f "$f"
    count=$((count+1))
  done < <(find public/ -type f \( -name "*.svg" -o -name "*.webp" -o -name "*.png" -o -name "*.jpg" -o -name "*.js" \))
  ok "${count} file(s) compressed with brotli"
fi

# --- Local build of the Rust app (Docker image) --------------------------------------
if [[ "$OPT_BUILD" == true ]]; then
  if [[ "$OPT_REBUILD" == true ]]; then
    step "Building image locally (no cache)"
    info "Recompiles everything from scratch. Estimated ~10-15 min."
    timer_start
    docker compose build --no-cache
  else
    step "Building image locally (incremental)"
    info "Only changed layers will be rebuilt. Estimated ~2-5 min."
    timer_start
    docker compose build
  fi
  timer_end
  ok "Build complete"

  step "Tagging image"
  docker tag "${IMAGE_NAME}:latest" "${IMAGE_NAME}:${GIT_TAG}" 2>/dev/null \
    || warn "Could not tag ${IMAGE_NAME}:${GIT_TAG} - check that 'image:' in docker-compose.yaml is ${IMAGE_NAME}:latest"
  ok "Image tagged ${IMAGE_NAME}:${GIT_TAG} and :latest"

  # --- Push to registry on LXC104 -----------------------------------------------------
  step "Pushing image to registry (${REGISTRY_HOST} / LXC104)"
  timer_start
  docker compose push \
    || fail "Push to ${REGISTRY_HOST} failed.\n      If the registry is plain HTTP, add \"${REGISTRY_HOST}\" to\n      insecure-registries in /etc/docker/daemon.json on this machine, then restart Docker."
  docker push "${IMAGE_NAME}:${GIT_TAG}" 2>/dev/null || true
  timer_end
  ok "Image available on ${REGISTRY_HOST}"
fi

# --- Sync docker-compose.yaml to LXC100 ------------------------------------------------
step "Syncing ${COMPOSE_FILE} to LXC100"
[[ ! -f "${COMPOSE_FILE}" ]] && fail "File ${COMPOSE_FILE} not found locally."
$SSH "$REMOTE" "mkdir -p ${REMOTE_DIR}"
$SCP "${COMPOSE_FILE}" "${REMOTE}:${REMOTE_DIR}/${COMPOSE_FILE}"
ok "${COMPOSE_FILE} synced"

# --- Pull image on LXC100 --------------------------------------------------------------
step "Pulling image on LXC100 (from ${REGISTRY_HOST})"
timer_start
$SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose pull" \
  || fail "Pull failed on LXC100.\n      If the registry is plain HTTP, add \"${REGISTRY_HOST}\" to\n      insecure-registries in /etc/docker/daemon.json on LXC100, then restart Docker."
timer_end
ok "Image pulled on LXC100"

# --- Deploy --------------------------------------------------------------------------
step "Starting services (LXC100)"
$SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose up -d"
ok "Containers started"

# --- Health check ----------------------------------------------------------------------
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

# --- Summary -----------------------------------------------------------------------------
echo ""
echo "=============================================="
echo "  Deployment successful"
echo "=============================================="
echo "  Image            : ${IMAGE_NAME}:${GIT_TAG}"
echo "  Container status : ./deploy.sh --status"
echo "  Live logs        : ./deploy.sh --logs"

# --- Discord notification -------------------------------------------------------------------
DISCORD_URL=$($SSH "$REMOTE" "grep -E '^DISCORD_WEBHOOK_URL=' ${REMOTE_DIR}/.env 2>/dev/null | cut -d= -f2-" || true)
if [[ -n "$DISCORD_URL" ]]; then
  $SSH "$REMOTE" "curl -sf -X POST '${DISCORD_URL}' \
    -H 'Content-Type: application/json' \
    -d '{\"content\":\"${APP_NAME} deployed successfully\"}' > /dev/null" \
    && echo "  Discord notified"
fi

echo ""

# --- Logs ------------------------------------------------------------------------------------
if [[ "$OPT_LOGS" == true ]]; then
  echo "Streaming logs - Ctrl+C to exit"
  echo ""
  $SSH "$REMOTE" "cd ${REMOTE_DIR} && docker compose logs -f"
fi