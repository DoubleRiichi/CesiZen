#!/bin/sh
set -e

# Premier argument = commande à exécuter (par défaut : "serve")
COMMAND="${1:-serve}"

case "$COMMAND" in
  migrate)
    echo "⏳ Waiting for database..."
    until sqlx migrate run --database-url "$DATABASE_URL"; do
      echo "Migration failed, retrying in 2s..."
      sleep 2
    done
    echo "✅ Migrations done."
    ;;

  serve)
    echo "🚀 Starting API..."
    exec ./api
    ;;

  migrate-and-serve)
    echo "⏳ Waiting for database..."
    until sqlx migrate run --database-url "$DATABASE_URL"; do
      echo "Migration failed, retrying in 2s..."
      sleep 2
    done
    echo "✅ Migrations done, starting API..."
    exec ./api
    ;;

  *)
    echo "Unknown command: $COMMAND" >&2
    echo "Usage: entrypoint.sh [migrate|serve|migrate-and-serve]" >&2
    exit 1
    ;;
esac