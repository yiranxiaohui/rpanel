### 1️⃣ 创建 migrations
```bash
cargo install sqlx-cli
sqlx migrate add init
```

### 生成实体Entity
  ```bash
  sea-orm-cli generate entity -u sqlite://data/app.db -o migration/src/entity --with-serde both
  ```