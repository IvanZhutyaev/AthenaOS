# Athena OS

Децентрализованная операционная система для цифрового суверенитета.

## Описание

Athena OS — это принципиально новая парадигма взаимодействия человека с цифровым миром, где пользователь является полновластным владельцем своих данных. Система сочетает мощь распределенных систем, семантических технологий, локального искусственного интеллекта и гуманистического дизайна.

## Архитектура

Проект состоит из следующих основных компонентов:

- **athena-core** - Ядро системы, инициализация и координация компонентов
- **athena-graph** - Графовый движок (Personal Knowledge Graph)
- **athena-security** - Система безопасности и криптография
- **athena-agents** - Агентная платформа (WASM runtime)
- **athena-sync** - P2P синхронизация (Athena Sync Protocol)
- **athena-api** - REST API сервер
- **athena-cli** - Командная строка

## Установка

### Требования

- Rust 1.70+
- Cargo

### Сборка

```bash
cargo build --release
```

## Использование

### Инициализация

```bash
cargo run --bin athena -- init --data-dir ~/.athena
```

### Запуск сервера

```bash
cargo run --bin athena -- start --port 8080
```

### CLI команды

```bash
# Создать узел в графе
cargo run --bin athena -- create-node --label "My Note"

# Список узлов
cargo run --bin athena -- list-nodes
```

## API

После запуска сервера доступны следующие endpoints:

- `GET /api/v1/health` - Проверка здоровья
- `GET /api/v1/nodes` - Список узлов
- `POST /api/v1/nodes` - Создать узел
- `GET /api/v1/nodes/:id` - Получить узел
- `DELETE /api/v1/nodes/:id` - Удалить узел
- `GET /api/v1/edges` - Список связей
- `POST /api/v1/edges` - Создать связь
- `POST /api/v1/query` - Запрос к графу
- `GET /api/v1/agents` - Список агентов
- `POST /api/v1/agents` - Загрузить агента
- `DELETE /api/v1/agents/:id` - Выгрузить агента

## Разработка

Проект использует workspace структуру. Для работы с отдельными компонентами:

```bash
# Сборка конкретного компонента
cargo build -p athena-graph

# Тесты
cargo test

# Документация
cargo doc --open
```

## Лицензия

MIT OR Apache-2.0
