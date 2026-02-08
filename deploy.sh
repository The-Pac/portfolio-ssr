#!/bin/bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${GREEN}=====================================${NC}"
echo -e "${GREEN} Portfolio SSR Deployment with SSL${NC}"
echo -e "${GREEN}=====================================${NC}\n"

APP_NAME="portfolio-ssr"
CERT_PATH="${TLS_CERT_PATH:-./public/arsac-baptiste.dev-ssl-bundle/domain.cert.pem}"
KEY_PATH="${TLS_KEY_PATH:-./public/arsac-baptiste.dev-ssl-bundle/private.key.pem}"
USE_SSL="${USE_SSL:-true}"
PORT="${PORT:-4006}"
SITE_ADDR="0.0.0.0:${PORT}"

show_help() {
  echo "Usage: ./deploy.sh [OPTIONS]"
  echo ""
  echo "Options:"
  echo "  --no-ssl         Start in HTTP mode only (port 8080)"
  echo "  --port PORT      Custom port (default: 443)"
  echo "  --skip-build     Skip rebuild, use existing binary"
  echo "  --incremental    Incremental build (only recompile changes)"
  echo "  --clean          Force full clean rebuild"
  echo "  -h, --help       Show this help message"
  echo ""
  echo "Examples:"
  echo "  ./deploy.sh                      Build and start with SSL"
  echo "  ./deploy.sh --no-ssl             Start in HTTP on port 8080"
  echo "  ./deploy.sh --port 4006 --no-ssl HTTP on port 4006"
  echo "  ./deploy.sh --skip-build         Use existing build"
  echo "  ./deploy.sh --incremental        Fast rebuild (only changes)"
  echo "  ./deploy.sh --clean              Force full clean rebuild"
}

SKIP_BUILD=false
INCREMENTAL_BUILD=false
CLEAN_BUILD=false

while [[ $# -gt 0 ]]; do
  case $1 in
    --no-ssl)
      USE_SSL=false
      PORT=8080
      SITE_ADDR="0.0.0.0:${PORT}"
      shift
      ;;
    --port)
      PORT="$2"
      SITE_ADDR="0.0.0.0:${PORT}"
      shift 2
      ;;
    --skip-build)
      SKIP_BUILD=true
      shift
      ;;
    --incremental)
      INCREMENTAL_BUILD=true
      shift
      ;;
    --clean)
      CLEAN_BUILD=true
      shift
      ;;
    -h|--help)
      show_help
      exit 0
      ;;
    *)
      echo -e "${RED}Unknown option: $1${NC}"
      show_help
      exit 1
      ;;
  esac
done

if [ "$PORT" -lt 1024 ] && [ "$EUID" -ne 0 ]; then
  echo -e "${RED}Error: Ports < 1024 require root privileges${NC}"
  echo -e "${YELLOW}Run with: sudo -E ./deploy.sh${NC}"
  exit 1
fi

if [ "$SKIP_BUILD" = false ]; then
  if [ "$INCREMENTAL_BUILD" = true ]; then
    echo -e "${CYAN}[1/4] Incremental build (compiling only changes)...${NC}"
  else
    echo -e "${CYAN}[1/4] Building application in release mode...${NC}"
  fi

  if ! command -v cargo-leptos &> /dev/null; then
    echo -e "${YELLOW}cargo-leptos not installed. Installing...${NC}"
    cargo install cargo-leptos
  fi

  if [ "$CLEAN_BUILD" = true ]; then
    echo -e "${CYAN}Full clean rebuild requested...${NC}"
    cargo clean 2>/dev/null || true
  elif [ "$INCREMENTAL_BUILD" = false ]; then
    echo -e "${CYAN}Cleaning previous builds...${NC}"
    cargo clean 2>/dev/null || true
  fi

  echo -e "${CYAN}Compiling...${NC}"
  cargo leptos build --release

  if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build successful${NC}\n"
  else
    echo -e "${RED}Build failed${NC}"
    exit 1
  fi
else
  echo -e "${YELLOW}[1/4] Build skipped (--skip-build)${NC}\n"
fi

if [ ! -f "target/release/${APP_NAME}" ]; then
  echo -e "${RED}Binary not found: target/release/${APP_NAME}${NC}"
  echo -e "${YELLOW}Run without --skip-build to compile${NC}"
  exit 1
fi

if [ "$USE_SSL" = true ]; then
  echo -e "${CYAN}[2/4] Verifying SSL certificates...${NC}"

  if [ ! -f "$CERT_PATH" ]; then
    echo -e "${RED}Certificate not found: $CERT_PATH${NC}"
    exit 1
  fi

  if [ ! -f "$KEY_PATH" ]; then
    echo -e "${RED}Private key not found: $KEY_PATH${NC}"
    exit 1
  fi

  KEY_PERMS=$(stat -c %a "$KEY_PATH" 2>/dev/null || stat -f %A "$KEY_PATH" 2>/dev/null)
  if [ "$KEY_PERMS" != "600" ] && [ "$KEY_PERMS" != "400" ]; then
    echo -e "${YELLOW}Key permissions: $KEY_PERMS (should be 600 or 400)${NC}"
  fi

  echo -e "${GREEN}Certificates valid${NC}"
else
  echo -e "${YELLOW}[2/4] HTTP mode (no SSL)${NC}\n"
fi

echo -e "${CYAN}[3/4] Configuring environment...${NC}"
export RUST_LOG="info"
export LEPTOS_SITE_ADDR="$SITE_ADDR"
export LEPTOS_SITE_ROOT="target/site"

if [ "$USE_SSL" = true ]; then
  export USE_TLS="true"
  export TLS_CERT_PATH="$CERT_PATH"
  export TLS_KEY_PATH="$KEY_PATH"
fi

echo -e "${CYAN}LEPTOS_SITE_ADDR=$LEPTOS_SITE_ADDR${NC}"
echo -e "${CYAN}LEPTOS_SITE_ROOT=$LEPTOS_SITE_ROOT${NC}"
echo -e "${CYAN}USE_TLS=${USE_TLS:-false}${NC}"
echo -e "${CYAN}RUST_LOG=$RUST_LOG${NC}\n"

if [ ! -d "target/site" ]; then
  echo -e "${RED}Directory target/site not found${NC}"
  exit 1
fi

echo -e "${CYAN}[4/4] Starting server...${NC}\n"

if [ "$USE_SSL" = true ]; then
  echo -e "${GREEN}=====================================${NC}"
  echo -e "${GREEN} HTTPS Server Started${NC}"
  echo -e "${GREEN}=====================================${NC}"
  echo -e "${GREEN}🔗 URL: https://0.0.0.0:${PORT}${NC}"
else
  echo -e "${GREEN}=====================================${NC}"
  echo -e "${GREEN} HTTP Server Started${NC}"
  echo -e "${GREEN}=====================================${NC}"
  echo -e "${GREEN}🔗 URL: http://0.0.0.0:${PORT}${NC}"
fi

echo -e "${YELLOW}Press Ctrl+C to stop${NC}\n"
echo -e "${YELLOW}Logs below...${NC}"

exec ./target/release/${APP_NAME}