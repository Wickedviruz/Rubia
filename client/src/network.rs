use bevy::prelude::*;
use crate::messages::{ClientMessage, ServerMessage, PlayerPosition};
use crate::PendingChunks;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::collections::HashMap;
use bincode;
use crossbeam_channel::{self, Receiver, Sender};
use std::thread;

#[derive(Resource)]
pub struct NetworkChannels {
    pub to_network_thread: Sender<ClientMessage>,
    pub from_network_thread: Receiver<ServerMessage>,
    pub other_players: HashMap<String, PlayerPosition>,
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let (to_net_tx, to_net_rx) = crossbeam_channel::unbounded::<ClientMessage>();
        let (from_net_tx, from_net_rx) = crossbeam_channel::unbounded::<ServerMessage>();

        thread::spawn(move || network_thread_loop(to_net_rx, from_net_tx));

        app.insert_resource(NetworkChannels {
            to_network_thread: to_net_tx,
            from_network_thread: from_net_rx,
            other_players: HashMap::new(),
        })
        .add_systems(Update, handle_incoming_messages)
        .add_systems(Update, (
            send_player_position,
            request_chunks_around_player,
        ));
    }
}

fn network_thread_loop(
    receiver: Receiver<ClientMessage>,
    sender: Sender<ServerMessage>,
) {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to server");
    stream.set_nonblocking(false).expect("Failed to set blocking mode");

    let mut buffer = vec![0; 2048];

    loop {
        // Skicka meddelanden från Bevy (main thread) till servern
        while let Ok(message) = receiver.try_recv() {
            let _ = stream.write_all(&bincode::serialize(&message).unwrap());
        }

        // Läs inkommande meddelanden från servern
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                if let Ok(message) = bincode::deserialize::<ServerMessage>(&buffer[..n]) {
                    let _ = sender.send(message);  // Skicka till Bevy
                }
            }
            _ => {}
        }

        // Enkel throttling för att inte bränna CPU
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

pub fn send_player_position(
    player_query: Query<&Transform, With<crate::player::Player>>,
    time: Res<Time>,
    net_channels: Res<NetworkChannels>,
    mut last_sent: Local<f32>,
) {
    *last_sent += time.delta_seconds();

    if *last_sent >= 0.1 {
        *last_sent = 0.0;

        if let Ok(transform) = player_query.get_single() {
            let position = PlayerPosition {
                x: transform.translation.x,
                y: transform.translation.y,
                z: transform.translation.z,
            };

            let message = ClientMessage::PlayerPositionUpdate(position);
            let _ = net_channels.to_network_thread.send(message);
        }
    }
}

pub fn request_chunks_around_player(
    player_query: Query<&Transform, With<crate::player::Player>>,
    net_channels: Res<NetworkChannels>,
) {
    if let Ok(transform) = player_query.get_single() {
        let cx = (transform.translation.x / 32.0).floor() as i32;
        let cy = (transform.translation.z / 32.0).floor() as i32;

        let message = ClientMessage::RequestChunks { center_x: cx, center_y: cy };
        let _ = net_channels.to_network_thread.send(message);
    }
}

pub fn handle_incoming_messages(
    mut net_channels: ResMut<NetworkChannels>,
    mut pending_chunks: ResMut<PendingChunks>,
) {
    while let Ok(message) = net_channels.from_network_thread.try_recv() {
        match message {
            ServerMessage::PlayerPositions(positions) => {
                net_channels.other_players.clear();
                for (id, pos) in positions {
                    net_channels.other_players.insert(id, pos);
                }
            }
            ServerMessage::ChunkData { x, y, blocks } => {
                pending_chunks.0.push((x, y, blocks));
            }
            _ => {}
        }
    }
}
