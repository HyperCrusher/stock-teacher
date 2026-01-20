use crate::{errors::StError, models::Player};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use uuid::Uuid;

pub fn get_player(
    conn: &PooledConnection<SqliteConnectionManager>,
    player: Uuid,
) -> Result<Player, StError> {
    let mut stmt = conn.prepare("SELECT id FROM players WHERE id=?1")?;
    let conn_iter = stmt.query_map(params![player], |row| {
        Ok(Player {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    let players: Vec<Player>;
    players = conn_iter.collect::<Result<Vec<Player>, _>>()?;
    // Prob should just have used query_one, but I like the ergonomics this provides
    // maybe I can learn how to override messages for next project
    if players.is_empty() {
        return Err(StError::DbNotFound("Player".to_string(), player));
    } else if players.len() > 1 {
        return Err(StError::DbNotUnique(
            players.len(),
            "Players".to_string(),
            player,
        ));
    }

    Ok::<_, crate::errors::StError>(players.first().unwrap().clone())
}

pub fn insert_player(
    conn: &PooledConnection<SqliteConnectionManager>,
    player: Player,
) -> Result<Uuid, StError> {
    conn.execute(
        "insert into players (id, name) values (?1, ?2)",
        params![player.id, player.name],
    )?;
    Ok::<_, crate::errors::StError>(player.id)
}
