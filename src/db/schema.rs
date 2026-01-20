use rusqlite::{Connection, Result};

pub fn init(conn: &Connection) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    for (name, sql) in TABLES {
        println!("Verifying table: {}", name);
        conn.execute(sql, [])?;
    }
    println!("All tables verfied!");
    Ok(())
}

const TABLES: &[(&str, &str)] = &[
    (
        "players",
        "
        CREATE TABLE IF NOT EXISTS players (
            id BLOB PRIMARY KEY,
            name TEXT NOT NULL
        );
        ",
    ),
    (
        "games",
        "
        CREATE TABLE IF NOT EXISTS games (
            id BLOB PRIMARY KEY,
            gm BLOB,
            password TEXT,
            FOREIGN KEY(gm) REFERENCES players(id)
        );
        ",
    ),
    (
        "game_rosters",
        "
        CREATE TABLE IF NOT EXISTS game_rosters (
            player_id BLOB,
            game_id BLOB,
            active BOOLEAN,
            PRIMARY KEY (player_id, game_id),
            FOREIGN KEY(player_id) REFERENCES players(id),
            FOREIGN KEY(game_id) REFERENCES games(id)
        );
        ",
    ),
    (
        "stocks",
        "
        CREATE TABLE IF NOT EXISTS stocks (
            id BLOB PRIMARY KEY,
            ticker TEXT NOT NULL,
            name TEXT
        );
        ",
    ),
    (
        "portfolios",
        "
        CREATE TABLE IF NOT EXISTS portfolios (
            player_id BLOB,
            stock_id BLOB,
            game_id BLOB,
            invest_amount TEXT,
            shares TEXT,
            PRIMARY KEY (player_id, stock_id, game_id),
            FOREIGN KEY(player_id) REFERENCES players(id),
            FOREIGN KEY(game_id) REFERENCES games(id),
            FOREIGN KEY(stock_id) REFERENCES stocks(id)
        );
        ",
    ),
    (
        "records",
        r#"
        CREATE TABLE IF NOT EXISTS records (
            id BLOB PRIMARY KEY,
            stock_id BLOB,
            date TEXT,
            open TEXT,
            close TEXT,
            FOREIGN KEY(stock_id) REFERENCES stocks(id)
        );
        "#,
    ),
    (
        "months",
        "
        CREATE TABLE IF NOT EXISTS months (
            id BLOB PRIMARY KEY,
            start TEXT,
            end TEXT,
            stock_id BLOB,
            open TEXT,
            close TEXT,
            FOREIGN KEY(stock_id) REFERENCES stocks(id)
        );
        ",
    ),
    (
        "weeks",
        "
        CREATE TABLE IF NOT EXISTS weeks (
            id BLOB PRIMARY KEY,
            stock_id BLOB,
            month_id BLOB,
            date TEXT,
            week_num INTEGER,
            peak TEXT,
            daysUp INTEGER,
            daysDown INTEGER,
            open TEXT,
            close TEXT,
            FOREIGN KEY(stock_id) REFERENCES stocks(id),
            FOREIGN KEY(month_id) REFERENCES months(id)
        );
        ",
    ),
    (
        "report_cards",
        "
        CREATE TABLE IF NOT EXISTS report_cards (
            player_id BLOB,
            stock_id BLOB,
            week_id BLOB,
            open TEXT,
            close TEXT,
            cached TEXT,
            PRIMARY KEY (player_id, stock_id, week_id),
            FOREIGN KEY(player_id) REFERENCES players(id),
            FOREIGN KEY(stock_id) REFERENCES stocks(id),
            FOREIGN KEY(week_id) REFERENCES weeks(id)
        );
        ",
    ),
];
