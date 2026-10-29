# Ethereum Smart Contract Framework (Rust + ethers-rs)

Фреймворк для взаимодействия со смарт-контрактами EVM: **Rust + ethers-rs + SOLID + Tokio**.

## Требования

- Rust **1.75+** (рекомендуется 1.80+)
- `cargo`

## Быстрый старт

```bash
# 1. Распакуйте архив
tar -xzf eth_framework.tar.gz
cd eth_framework

# 2. ОБЯЗАТЕЛЬНО удалите старый lock и кэш scale-крейтов
rm -f Cargo.lock
cargo clean

# 3. Сборка
cargo build

# 4. Запуск демо
cargo run
```

## Если ошибка `cannot find __private in scale`

```bash
rm -f Cargo.lock
rm -rf ~/.cargo/registry/src/*/scale-info-*
rm -rf ~/.cargo/registry/src/*/parity-scale-codec*
cargo build
```

В `Cargo.toml` уже зафиксированы совместимые версии:

```toml
parity-scale-codec = { version = "=3.6.9", features = ["derive", "full"] }
parity-scale-codec-derive = "=3.6.9"
scale-info = { version = "=2.11.0", features = ["derive"] }
```

## Архитектура (SOLID)

```
src/
├── core/           # Traits + Error          → DIP, ISP
├── providers/      # HTTP provider           → SRP, OCP
├── signers/        # LocalWallet             → SRP
├── contracts/      # ERC20 (abigen) + generic → SRP
├── services/       # EthService + ConcurrentMonitor
└── utils/
```

## Демо

1. Подключение к публичному Ethereum RPC
2. chain_id + номер блока
3. Параллельная проверка балансов (Tokio)
4. Type-safe вызов WETH (name, symbol, decimals, totalSupply)

## Отправка транзакций

Раскомментируйте секцию в `main.rs` и задайте:

```bash
export PRIVATE_KEY=0x...
cargo run
```

> **Примечание**: ethers-rs в maintenance. Для новых проектов — [Alloy](https://github.com/alloy-rs).
