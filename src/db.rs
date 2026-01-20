use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use uuid::{self, Uuid};

use crate::{errors::StError, models::*};

mod queries;
mod schema;

type SqlPool = Pool<SqliteConnectionManager>;

fn init(pool: &SqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get()?;
    schema::init(&conn)?;
    Ok(())
}

pub fn get_uuid() -> Uuid {
    Uuid::new_v4()
}

#[derive(Clone)]
pub struct DB {
    pool: SqlPool,
}

impl DB {
    pub fn new(path: &str) -> Self {
        let manager = SqliteConnectionManager::file(path);
        let pool = r2d2::Pool::new(manager).expect("Failed to Create Connection Pool");
        init(&pool).expect("Failed to initialize DB");
        Self { pool }
    }

    pub async fn run<F, T>(&self, task: F) -> Result<T, StError>
    where
        F: FnOnce(&PooledConnection<SqliteConnectionManager>) -> Result<T, StError>
            + Send
            + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get()?;
            let data = task(&conn)?;
            Ok(data)
        })
        .await?
    }

    pub async fn new_game(
        &self,
        player: Player,
        password: Option<&str>,
    ) -> Result<Uuid, crate::errors::StError> {
        let game = Game::new(&player.id, password)?;
        self.run(|conn| queries::insert_game(conn, game)).await
    }

    pub async fn get_active_games(&self) -> Result<Vec<Game>, crate::errors::StError> {
        self.run(|conn| queries::get_games(conn, Some(true))).await
    }

    pub async fn get_inactive_games(&self) -> Result<Vec<Game>, crate::errors::StError> {
        self.run(|conn| queries::get_games(conn, Some(false))).await
    }

    pub async fn get_all_games(&self) -> Result<Vec<Game>, crate::errors::StError> {
        self.run(|conn| queries::get_games(conn, None)).await
    }

    pub async fn new_player(&self, name: &str) -> Result<Uuid, crate::errors::StError> {
        let player = Player::new(name);
        self.run(|conn| queries::insert_player(conn, player)).await
    }

    pub async fn get_player(&self, id: Uuid) -> Result<Uuid, crate::errors::StError> {
        self.run(|conn| queries::get_player(conn, id)).await
    }
}
