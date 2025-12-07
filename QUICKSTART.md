# Athena OS - Quick Start Guide

## Быстрый старт

### 1. Инициализация проекта

```bash
# Инициализировать Athena OS
cargo run --bin athena -- init --data-dir ~/.athena
```

### 2. Запуск сервера

```bash
# Запустить API сервер на порту 8080
cargo run --bin athena -- start --port 8080
```

### 3. Запуск Frontend

В отдельном терминале:

```bash
cd frontend
npm install
npm run dev
```

Frontend будет доступен на http://localhost:3000

### 4. Использование CLI

```bash
# Создать узел в графе
cargo run --bin athena -- create-node --label "My First Note"

# Список всех узлов
cargo run --bin athena -- list-nodes
```

## API Примеры

### Создать узел

```bash
curl -X POST http://localhost:8080/api/v1/nodes \
  -H "Content-Type: application/json" \
  -d '{"label": "Test Node", "properties": {}}'
```

### Получить список узлов

```bash
curl http://localhost:8080/api/v1/nodes
```

### Создать связь

```bash
curl -X POST http://localhost:8080/api/v1/edges \
  -H "Content-Type: application/json" \
  -d '{
    "from": "node-uuid-1",
    "to": "node-uuid-2",
    "label": "relates_to"
  }'
```

## Структура проекта

```
AthenaOS/
├── athena-core/          # Ядро системы
├── athena-graph/         # Графовый движок
├── athena-security/      # Безопасность
├── athena-agents/        # Агентная платформа
├── athena-sync/          # P2P синхронизация
├── athena-api/           # REST API
├── athena-cli/           # CLI инструмент
└── frontend/             # React frontend
```

## Следующие шаги

1. Изучите [README.md](README.md) для подробной документации
2. Посмотрите примеры использования в [AthenaOS.txt](AthenaOS.txt)
3. Начните создавать свои агенты и узлы в графе знаний

