mod messages;
mod world;

use messages::{ClientMessage, ServerMessage, PlayerPosition};
use world::Chunk;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, broadcast};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use std::sync::Arc;
use bincode;

type PlayerMap = Arc<Mutex<HashMap<String, (Arc<Mutex<TcpStream>>, PlayerPosition)>>>;
type ChunkMap = Arc<Mutex<HashMap<(i32, i32), Chunk>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    println!("Server running on 127.0.0.1:7878");

    let players: PlayerMap = Arc::new(Mutex::new(HashMap::new()));
    let world = Arc::new(Mutex::new(generate_world()));

    let (tx, _rx) = broadcast::channel::<HashMap<String, PlayerPosition>>(16);

    let players_clone = players.clone();
    tokio::spawn(broadcast_positions_loop(players_clone, tx.clone()));

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("New connection from {}", addr);

        let players_clone = players.clone();
        let world_clone = world.clone();
        let tx_clone = tx.clone();

        tokio::spawn(handle_connection(socket, players_clone, world_clone, tx_clone));
    }
}

async fn handle_connection(
    socket: TcpStream,
    players: PlayerMap,
    world: ChunkMap,
    tx: broadcast::Sender<HashMap<String, PlayerPosition>>,
) {
    let player_id = socket.peer_addr().unwrap().to_string();
    let player_stream = Arc::new(Mutex::new(socket));

    players.lock().await.insert(player_id.clone(), (player_stream.clone(), PlayerPosition { x: 16.0, y: 1.0, z: 16.0 }));

    let mut buffer = [0; 2048];

    loop {
        let n = {
            let mut stream = player_stream.lock().await;
            match stream.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            }
        };

        if let Ok(message) = bincode::deserialize::<ClientMessage>(&buffer[..n]) {
            match message {
                ClientMessage::RequestChunks { center_x, center_y } => {
                    send_chunks_around(player_stream.clone(), world.clone(), center_x, center_y).await;
                }
                ClientMessage::PlayerPositionUpdate(pos) => {
                    players.lock().await.entry(player_id.clone()).and_modify(|(_, p)| *p = pos);
                    broadcast_positions(players.clone(), &tx).await;
                }
                ClientMessage::Connect { player_name } => {
                    println!("Player {} connected as '{}'", player_id, player_name);
                }
                _ => {}
            }
        }
    }

    players.lock().await.remove(&player_id);
    println!("Player {} disconnected", player_id);
}

async fn send_chunks_around(
    stream: Arc<Mutex<TcpStream>>,
    world: ChunkMap,
    center_x: i32,
    center_y: i32,
) {
    let world = world.lock().await;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if let Some(chunk) = world.get(&(center_x + dx, center_y + dy)) {
                let message = ServerMessage::ChunkData {
                    x: center_x + dx,
                    y: center_y + dy,
                    blocks: chunk.to_flat_vec(),
                };
                let encoded = bincode::serialize(&message).unwrap();

                let _ = stream.lock().await.write_all(&encoded).await;
            }
        }
    }
}

async fn broadcast_positions(players: PlayerMap, tx: &broadcast::Sender<HashMap<String, PlayerPosition>>) {
    let players_lock = players.lock().await;

    let positions: HashMap<String, PlayerPosition> = players_lock.iter()
        .map(|(id, (_, pos))| (id.clone(), *pos))
        .collect();

    let _ = tx.send(positions.clone());

    for (_, (stream, _)) in players_lock.iter() {
        let message = ServerMessage::PlayerPositions(positions.clone().into_iter().collect());
        let encoded = bincode::serialize(&message).unwrap();
        let _ = stream.lock().await.write_all(&encoded).await;
    }
}

async fn broadcast_positions_loop(players: PlayerMap, tx: broadcast::Sender<HashMap<String, PlayerPosition>>) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(500));
    loop {
        interval.tick().await;
        broadcast_positions(players.clone(), &tx).await;
    }
}

fn generate_world() -> HashMap<(i32, i32), Chunk> {
    let mut world = HashMap::new();
    for x in -2..=2 {
        for y in -2..=2 {
            world.insert((x, y), Chunk::generate(x, y));
        }
    }
    world
}
