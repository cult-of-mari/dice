use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_tokio::{TokioPlugin, TokioRuntime};
use dice_codec::DiceCodec;
use dice_proto::DicePacket;
use futures_util::{SinkExt, StreamExt};
use std::io;
use std::iter;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_util::codec::Framed;

const TICK_RATE: Duration = Duration::from_secs(1).checked_div(20).unwrap();

#[derive(Resource, Deref, DerefMut)]
pub struct ClientReader(pub crossbeam_channel::Receiver<AcceptStream>);

pub struct AcceptStream(TcpStream, SocketAddr);

#[derive(Component)]
pub struct Client {
    inbound_reader: mpsc::UnboundedReceiver<DicePacket>,
    outbound_writer: mpsc::UnboundedSender<DicePacket>,
}

#[derive(Event)]
pub struct Handshake;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ScheduleRunnerPlugin::run_loop(TICK_RATE)),
            TokioPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (accept_clients, process_clients))
        .run();
}

fn setup(mut commands: Commands, tokio: Res<TokioRuntime>) {
    let (client_writer, client_reader) = crossbeam_channel::unbounded();

    tokio.spawn(start_server(client_writer));
    commands.insert_resource(ClientReader(client_reader));
}

fn accept_clients(
    mut commands: Commands,
    client_reader: Res<ClientReader>,
    tokio: Res<TokioRuntime>,
) {
    for AcceptStream(stream, _addr) in client_reader.try_iter() {
        let (inbound_writer, inbound_reader) = mpsc::unbounded_channel();
        let (outbound_writer, outbound_reader) = mpsc::unbounded_channel();

        tokio.spawn(process_client(stream, inbound_writer, outbound_reader));

        commands
            .spawn(Client {
                inbound_reader,
                outbound_writer,
            })
            .observe(|trigger: Trigger<Handshake>| {
                println!("handshook");
            });
    }
}

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::{B4, B8};

#[bitfield]
struct PackedPosition {
    x: B4,
    z: B4,
    y: B8,
}

fn pack_position(x: u16, y: u16, z: u16) -> i16 {
    let mut data = PackedPosition::new();

    data.set_x(x as u8);
    data.set_y(y as u8);
    data.set_z(z as u8);

    i16::from_ne_bytes(data.into_bytes())
}

fn process_clients(mut commands: Commands, mut query: Query<(Entity, &mut Client)>) {
    for (entity, mut client) in query.iter_mut() {
        while let Ok(event) = client.inbound_reader.try_recv() {
            match event {
                DicePacket::KeepAlive => {
                    client.outbound_writer.send(DicePacket::KeepAlive).unwrap();
                }
                DicePacket::Handshake { username } => {
                    let name = username.to_string_lossy();
                    let message = format!("{name} has joined the game.");

                    info!("{message}");

                    client
                        .outbound_writer
                        .send(DicePacket::Login {
                            protocol_version: 14,
                            username,
                            random_seed: 0,
                            dimension: 0,
                        })
                        .unwrap();

                    let mut blocks_position = Vec::new();
                    let mut blocks_id = Vec::new();
                    let mut blocks_metadata = Vec::new();

                    for x in 0..16 {
                        for z in 0..16 {
                            blocks_position.push(pack_position(x, 64, z));
                            blocks_id.push(0);
                            blocks_metadata.push(0);
                        }
                    }

                    client
                        .outbound_writer
                        .send(DicePacket::ChunkBlockSet {
                            chunk_x: 0,
                            chunk_y: 0,
                            blocks_len: blocks_position.len() as i16,
                            blocks_position,
                            blocks_id,
                            blocks_metadata,
                        })
                        .unwrap();

                    client
                        .outbound_writer
                        .send(DicePacket::SpawnPosition { x: 0, y: 64, z: 0 })
                        .unwrap();

                    client
                        .outbound_writer
                        .send(DicePacket::HumanSpawn {
                            entity_id: 0,
                            username: "player0".parse().unwrap(),
                            position: dice_proto::EntityPosition { x: 0, y: 64, z: 0 },
                            yaw: 0,
                            pitch: 0,
                            current_item: 0,
                        })
                        .unwrap();

                    commands.trigger_targets(Handshake, entity);
                }
                _ => {}
            }
        }
    }
}

async fn start_server(client_writer: crossbeam_channel::Sender<AcceptStream>) -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:25565").await?;
    let addr = listener.local_addr()?;

    info!("listening on {addr}");

    while let Ok((stream, addr)) = listener.accept().await {
        client_writer.send(AcceptStream(stream, addr)).unwrap();
    }

    Ok(())
}

async fn process_client(
    stream: TcpStream,
    inbound_writer: mpsc::UnboundedSender<DicePacket>,
    mut outbound_reader: mpsc::UnboundedReceiver<DicePacket>,
) -> io::Result<()> {
    let (mut outbound, mut inbound) = Framed::new(stream, DiceCodec).split();

    tokio::spawn(async move {
        while let Some(item) = outbound_reader.recv().await {
            debug!("outbound: {item:?}");
            outbound.send(item).await?;
        }

        io::Result::Ok(())
    });

    tokio::spawn(async move {
        while let Some(Ok(item)) = inbound.next().await {
            debug!("inbound: {item:?}");
            inbound_writer.send(item).unwrap();
        }

        io::Result::Ok(())
    });

    Ok(())
}
