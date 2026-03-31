#!/bin/bash
set -e

echo "⏳ Waiting for database..."
until sqlx migrate run --database-url "$DATABASE_URL"; do
  echo "Migration failed, retrying in 2s..."
  sleep 2
done

echo "✅ Migrations done, starting API..."
exec ./api