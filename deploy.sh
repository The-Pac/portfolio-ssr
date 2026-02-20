#!/bin/bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${GREEN}=====================================${NC}"
echo -e "${GREEN}  Portfolio SSR - Docker Deploy     ${NC}"
echo -e "${GREEN}=====================================${NC}\n"

APP_NAME="portfolio-ssr"

show_help() {
  echo "Usage: ./deploy.sh [OPTIONS]"
  echo ""
  echo "Options:"
  echo "  --build          Build intelligent (cache Cargo préservé)"
  echo "  --rebuild        Force rebuild complet (no-cache)"
  echo "  --fix-images     Corrige CRLF→LF + brotli sur les SVG/images"
  echo "  --down           Arrête les containers"
  echo "  --logs           Affiche les logs en live"
  echo "  -h, --help       Affiche cette aide"
}

FORCE_BUILD=false
FULL_REBUILD=false
FIX_IMAGES=false
DO_DOWN=false
SHOW_LOGS=false

while [[ $# -gt 0 ]]; do
  case $1 in
    --build)       FORCE_BUILD=true; shift ;;
    --rebuild)     FORCE_BUILD=true; FULL_REBUILD=true; shift ;;
    --fix-images)  FIX_IMAGES=true; shift ;;
    --down)        DO_DOWN=true; shift ;;
    --logs)        SHOW_LOGS=true; shift ;;
    -h|--help)     show_help; exit 0 ;;
    *)
      echo -e "${RED}Option inconnue: $1${NC}"
      show_help
      exit 1
      ;;
  esac
done

if [ ! -f ".env" ]; then
  echo -e "${RED}Fichier .env manquant !${NC}"
  echo -e "${YELLOW}Copie .env.example en .env et remplis les valeurs.${NC}"
  exit 1
fi

if [ "$DO_DOWN" = true ]; then
  echo -e "${CYAN}Arrêt des containers...${NC}"
  docker compose down
  echo -e "${GREEN}Containers arrêtés.${NC}"
  exit 0
fi

if [ "$FIX_IMAGES" = true ]; then
  echo -e "${CYAN}[FIX-IMAGES] Correction CRLF→LF sur les SVG...${NC}"
  find public/ -type f -name "*.svg" | while read f; do
    if file "$f" | grep -q CRLF; then
      sed -i 's/\r//' "$f"
      echo -e "${GREEN}  fixé: $f${NC}"
    fi
  done

  echo -e "${CYAN}[FIX-IMAGES] Compression brotli...${NC}"
  if ! command -v brotli &> /dev/null; then
    sudo apt-get install -y brotli
  fi
  find public/ -type f \( -name "*.svg" -o -name "*.webp" -o -name "*.png" -o -name "*.jpg" -o -name "*.js" \) | while read f; do
    brotli -q 11 -k -f "$f"
    echo -e "${GREEN}  brotli: $f${NC}"
  done
  echo -e "${GREEN}Images OK\n${NC}"
fi

if [ "$FORCE_BUILD" = true ]; then
  if [ "$FULL_REBUILD" = true ]; then
    echo -e "${CYAN}[REBUILD] Build complet sans cache...${NC}"
    echo -e "${YELLOW}(~10-15 min)${NC}\n"
    docker compose build --no-cache
  else
    echo -e "${CYAN}[BUILD] Build intelligent (cache Cargo préservé)...${NC}"
    echo -e "${YELLOW}(~2-5 min si seul le code a changé)${NC}\n"
    docker compose build
  fi
  echo -e "${GREEN}Build terminé\n${NC}"
fi

echo -e "${CYAN}[DEPLOY] Lancement des containers...${NC}"
docker compose up -d

echo -e "${GREEN}=====================================${NC}"
echo -e "${GREEN}  Déployé avec succès !              ${NC}"
echo -e "${GREEN}=====================================${NC}"
echo -e "${CYAN}Caddy actif → ton domaine est en ligne${NC}"
echo -e "${YELLOW}Pour voir les logs : ./deploy.sh --logs${NC}\n"

if [ -f ".env" ]; then
  source .env
fi
if [ -n "$DISCORD_WEBHOOK_URL" ]; then
  curl -s -X POST "$DISCORD_WEBHOOK_URL" \
    -H "Content-Type: application/json" \
    -d "$(printf '{"content": "**%s** deploye avec succes !"}' "${APP_NAME}")" \
    > /dev/null && echo -e "${GREEN}Notification Discord envoyée${NC}"
fi

if [ "$SHOW_LOGS" = true ]; then
  docker compose logs -f
fi