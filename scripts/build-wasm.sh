#!/bin/bash
# Сборка Rust-ядра в WebAssembly и копирование в web/

set -e

echo "📦 Сборка NoemaFeed Mobile (WASM)..."

# Проверяем наличие wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack не найден. Установите: https://rustwasm.github.io/wasm-pack/"
    exit 1
fi

# Собираем в режиме --target web (для PWA)
wasm-pack build --target web --out-dir ./web/pkg --release

# Копируем базы знаний в web/kb (для доступа из JS)
mkdir -p ./web/kb
cp -r ./kb/* ./web/kb/

echo "✅ Сборка завершена. Откройте web/index.html в браузере."