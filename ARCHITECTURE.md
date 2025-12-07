# Athena OS - Architecture Overview

## Общая архитектура

Athena OS построена по модульной архитектуре с четким разделением ответственности.

## Компоненты

### athena-core
Ядро системы, координирует работу всех компонентов:
- Инициализация системы
- Управление конфигурацией
- Координация между модулями

### athena-graph
Графовый движок для Personal Knowledge Graph:
- Хранение узлов и связей (RocksDB)
- Запросы к графу (GraphPattern)
- Версионирование данных
- CRUD операции

**API:**
```rust
trait GraphEngine {
    async fn query(&self, pattern: &GraphPattern) -> Result<QueryResult>;
    async fn update(&self, update: &GraphUpdate) -> Result<VersionId>;
    async fn get_node(&self, id: &NodeId) -> Result<Option<Entity>>;
    async fn put_node(&self, entity: Entity) -> Result<()>;
}
```

### athena-security
Система безопасности:
- Криптография (Ed25519, ChaCha20Poly1305)
- Управление ключами
- Система разрешений (Capability-based)

**Компоненты:**
- `PublicKey` / `PrivateKey` - криптографические ключи
- `KeyManager` - хранение и управление ключами
- `PermissionSet` - набор разрешений для агентов

### athena-agents
Агентная платформа:
- WASM runtime (wasmtime)
- Изоляция агентов
- Система разрешений
- API для агентов

**Структура агента:**
```rust
struct AthenaAgent {
    metadata: AgentMetadata,
    permissions: PermissionSet,
    wasm_module: Vec<u8>,
    // ...
}
```

### athena-sync
P2P синхронизация:
- libp2p для сетевого взаимодействия
- Athena Sync Protocol
- CRDT для разрешения конфликтов
- E2E шифрование

**Протокол:**
- Graph Updates - синхронизация графа
- Presence - статус узлов
- Messages - децентрализованный мессенджер

### athena-api
REST API сервер:
- Axum framework
- GraphQL (планируется)
- WebSocket для событий
- CORS поддержка

**Endpoints:**
- `/api/v1/nodes` - управление узлами
- `/api/v1/edges` - управление связями
- `/api/v1/query` - запросы к графу
- `/api/v1/agents` - управление агентами

### athena-cli
Командная строка:
- Инициализация системы
- Управление узлами
- Запуск сервера
- Утилиты

## Потоки данных

### Создание узла
1. CLI/API → athena-api
2. athena-api → athena-core
3. athena-core → athena-graph
4. athena-graph → RocksDB

### Синхронизация
1. Локальные изменения → athena-graph
2. athena-graph → athena-sync (CRDT)
3. athena-sync → P2P сеть
4. Удаленные узлы получают обновления

### Загрузка агента
1. API → athena-api
2. athena-api → athena-core
3. athena-core → athena-agents
4. athena-agents → WASM runtime

## Безопасность

- Все данные шифруются на клиенте
- Ключи хранятся в зашифрованном виде
- Агенты изолированы в WASM sandbox
- P2P соединения используют E2E шифрование
- Capability-based security для агентов

## Масштабирование

- Граф поддерживает миллионы узлов
- RocksDB оптимизирован для больших объемов
- P2P сеть масштабируется горизонтально
- Агенты выполняются изолированно

## Будущие улучшения

- GraphQL API
- WebSocket для real-time обновлений
- Расширенная система индексов
- Оптимизация запросов
- Кэширование
- Распределенное хранение файлов

