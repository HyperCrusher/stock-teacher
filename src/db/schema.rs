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
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL
        );
        ",
    ),
    (
        "games",
        "
        CREATE TABLE IF NOT EXISTS games (
            id TEXT PRIMARY KEY,
            gm TEXT,
            password TEXT,
            FOREIGN KEY(gm) REFERENCES players(id)
        );
        ",
    ),
    (
        "game_rosters",
        "
        CREATE TABLE IF NOT EXISTS game_rosters (
            player_id TEXT,
            game_id TEXT,
            active BOOLEAN,
            FOREIGN KEY(player_id) REFERENCES players(id),
            FOREIGN KEY(game_id) REFERENCES games(id)
        );
        ",
    ),
    (
        "stocks",
        "
        CREATE TABLE IF NOT EXISTS stocks (
            id TEXT PRIMARY KEY,
            ticker TEXT NOT NULL,
            name TEXT
        );
        ",
    ),
    (
        "portfolios",
        "
        CREATE TABLE IF NOT EXISTS portfolios (
            player_id TEXT,
            stock_id TEXT,
            game_id TEXT,
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
            id TEXT PRIMARY KEY,
            stock_id TEXT,
            "date" DATE,
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
            id TEXT PRIMARY KEY,
            start DATE,
            end DATE,
            stock_id TEXT,
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
            id TEXT PRIMARY KEY,
            stock_id TEXT,
            month_id TEXT,
            date DATE,
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
        "reports",
        "
        CREATE TABLE IF NOT EXISTS report_cards (
            player_id TEXT,
            stock_id TEXT,
            week_id TEXT,
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
