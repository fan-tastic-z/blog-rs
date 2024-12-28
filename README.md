# README

## 主要使用到的rust库

- Axum
- Sqlx
- sqlx-adapter
- thiserror
- anyhow
- validator
- tracing
- jsonwebtoken
- argon2

## 本地开发调试

启动数据库：

```bash
sh ./scripts/init_db.sh
```

运行程序

```bash
cargo run
```

## 目前的情况

主要是通过 <https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust> 文章进行一个实践学习，但是目录结构上没有完全按照作者的仓库例子实现，还是使用自己习惯的`mod`的方式。
