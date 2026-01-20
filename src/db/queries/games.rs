use crate::{errors::StError, models::Game};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use uuid::Uuid;

pub fn get_games(
    conn: &PooledConnection<SqliteConnectionManager>,
    active: Option<bool>,
) -> Result<Vec<Game>, StError> {
    let mut stmt =
        conn.prepare("SELECT id, gm, active, password FROM games WHERE ?1 IS NULL OR active=?1")?;
    let conn_iter = stmt.query_map(params![active], |row| {
        Ok(Game {
            id: row.get(0)?,
            gm: row.get(1)?,
            active: row.get(2)?,
            password: row.get(3)?,
        })
    })?;

    let games: Result<Vec<Game>, _> = conn_iter.collect();
    Ok(games?)
}

pub fn insert_game(
    conn: &PooledConnection<SqliteConnectionManager>,
    game: Game,
) -> Result<Uuid, StError> {
    conn.execute(
        "insert into games (id, gm, active, password) values (?1, ?2, ?3, ?4)",
        params![game.id, game.gm, game.active, game.password],
    )?;
    Ok::<_, crate::errors::StError>(game.id)
}
