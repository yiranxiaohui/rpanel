## 运行 Migrator 命令行工具

* **生成一个新的迁移文件**

  ```sh
  cargo run -- generate MIGRATION_NAME
  ```

* **应用所有尚未执行的迁移**

  ```sh
  cargo run
  ```

  或

  ```sh
  cargo run -- up
  ```

* **仅应用前 10 个尚未执行的迁移**

  ```sh
  cargo run -- up -n 10
  ```

* **回滚最近一次已执行的迁移**

  ```sh
  cargo run -- down
  ```

* **回滚最近 10 次已执行的迁移**

  ```sh
  cargo run -- down -n 10
  ```

* **删除数据库中的所有表，然后重新执行所有迁移**

  ```sh
  cargo run -- fresh
  ```

* **回滚所有已执行的迁移，然后重新执行所有迁移**

  ```sh
  cargo run -- refresh
  ```

* **回滚所有已执行的迁移（不重新执行）**

  ```sh
  cargo run -- reset
  ```

* **查看所有迁移的执行状态**

  ```sh
  cargo run -- status
  ```