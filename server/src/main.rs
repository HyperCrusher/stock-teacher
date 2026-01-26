use warp::Filter;

use crate::models::{Game, Player};

mod auth;
mod db;
mod errors;
mod models;

#[tokio::main]
async fn main() {
    // let db: db::DB = db::DB::new("./test.db3");
    // for i in 0..7 {
    //     let gm = match i % 2 {
    //         0 => Player::new("Tim"),
    //         _ => Player::new("Sarah"),
    //     };
    //     let password = match i % 2 {
    //         0 => Some("password"),
    //         _ => None,
    //     };
    //     let game = Game::new(&gm.id, password).unwrap_or_else(|e| panic!("{e}"));
    //     println!(
    //         "Game:\n     id: {}\n     gm:{}\n     password:{}",
    //         game.id,
    //         game.gm,
    //         password.unwrap_or_default()
    //     );
    //     db.create_game(game, gm)
    //         .await
    //         .unwrap_or_else(|e| panic!("error creating game: {e}"));
    // }
    // let games = db.get_games().await.unwrap();
    // println!("{}", games.len());
    // for game in games {
    //     println!(
    //         "Game:\n     id: {}\n     gm:{}\n     password:{}",
    //         game.id,
    //         game.gm,
    //         game.password.unwrap_or_else(|| "[No Password]".to_string())
    //     )
    // }
    // let routes = warp::path!(String).map(|route| format!("Hello, {}!", route));
    // warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
