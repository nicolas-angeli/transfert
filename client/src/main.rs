use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct gameInfo {
	gameId: u32,
    player1: Position,
    player2: Position,
    ball: Position,
    gameStatus: i8,
    
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: i8,
    y: i8,
}

impl gameInfo {
	pub fn GameInfo() ->  Self {
		gameInfo {
			gameId: 0,
			player1: Position{
				x: 0,
				y: 0,
			},
			player2: Position{
				x: 0,
				y: 0,
			},
			ball: Position{
				x: 0,
				y: 0,
			},
			gameStatus: 0,
		}
	}
}

use tokio::time;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let gamestate = gameInfo::GameInfo();

    let client = reqwest::Client::new();
    let res = client.post("https://433b-89-90-162-43.ngrok-free.app")
        .json(&gamestate)
        .send()
        .await?;

    let opponent: Position = res.json().await?; // Parse the response body as JSON into a GameState
    
    println!("Response: {:?}", opponent); 

    Ok(())
}


