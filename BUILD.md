# Athena OS - Build Instructions

## Требования

- Rust 1.70 или выше
- Cargo (обычно идет с Rust)
- Node.js 18+ и npm (для frontend)
- RocksDB системные библиотеки

### Установка зависимостей

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install build-essential libclang-dev librocksdb-dev
```

#### macOS
```bash
brew install rocksdb
```

#### Windows
Установите Visual Studio Build Tools и vcpkg для RocksDB.

## Сборка

### Backend (Rust)

```bash
# Сборка всех компонентов
cargo build --release

# Сборка конкретного компонента
cargo build -p athena-core --release

# Сборка с оптимизациями
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Frontend (React/TypeScript)

```bash
cd frontend
npm install
npm run build
```

## Тестирование

```bash
# Запуск всех тестов
cargo test

# Тесты конкретного компонента
cargo test -p athena-graph

# Тесты с выводом
cargo test -- --nocapture
```

## Разработка

### Запуск в режиме разработки

```bash
# Backend
cargo run --bin athena -- start --port 8080

# Frontend (в отдельном терминале)
cd frontend
npm run dev
```

### Генерация документации

```bash
cargo doc --open
```

## Troubleshooting

### Ошибки компиляции RocksDB

Убедитесь, что установлены системные библиотеки RocksDB.

### Проблемы с libp2p

Проверьте версию Rust - требуется 1.70+.

### Frontend не подключается к API

Убедитесь, что:
1. Backend запущен на порту 8080
2. В `frontend/vite.config.ts` правильно настроен proxy

