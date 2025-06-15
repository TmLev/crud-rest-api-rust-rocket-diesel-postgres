# CRUD REST API [Rust + Rocket + Diesel + Postgres]

> [!CAUTION]
> I don't encourage using Rocket in production.  
> I've switched to Axum + sqlx and suggest you do the same.  
> Diesel is totally fine, I just prefer sqlx.

API should allow simple CRUD operations over `artists` entities.

Current status:
- [x] List
- [x] Retrieve
- [x] Create
- [x] Update
- [x] Destroy
