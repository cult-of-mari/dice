use self::protocol::{MinecraftBool, MinecraftPacket};
use self::runtime::{TokioPlugin, TokioRuntime};
use bevy::prelude::*;
use futures_util::{SinkExt, StreamExt};
use protocol::MinecraftString;
use std::io;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_util::codec::Framed;

mod protocol;
mod runtime;

#[derive(Resource, Deref, DerefMut)]
pub struct ClientReader(pub crossbeam_channel::Receiver<AcceptStream>);

pub struct AcceptStream(TcpStream, SocketAddr);

#[derive(Component)]
pub struct Client {
    inbound_reader: mpsc::UnboundedReceiver<MinecraftPacket>,
    outbound_writer: mpsc::UnboundedSender<MinecraftPacket>,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TokioPlugin))
        .add_event::<MinecraftPacket>()
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

        commands.spawn(Client {
            inbound_reader,
            outbound_writer,
        });
    }
}

fn process_clients(mut commands: Commands, mut query: Query<(Entity, &mut Client)>) {
    for (entity, mut client) in query.iter_mut() {
        while let Ok(event) = client.inbound_reader.try_recv() {
            match event {
                MinecraftPacket::Handshake { username } => {
                    let name = String::from(username.clone());
                    let message = format!("{name} has joined the game.");

                    info!("{message}");

                    client
                        .outbound_writer
                        .send(MinecraftPacket::Login {
                            version: 14,
                            username,
                            seed: 0,
                            dimension: 0,
                        })
                        .unwrap();

                    client
                        .outbound_writer
                        .send(MinecraftPacket::Chat {
                            message: MinecraftString::from(message),
                        })
                        .unwrap();
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
    inbound_writer: mpsc::UnboundedSender<MinecraftPacket>,
    mut outbound_reader: mpsc::UnboundedReceiver<MinecraftPacket>,
) -> io::Result<()> {
    let (mut outbound, mut inbound) = Framed::new(stream, protocol::MinecraftCodec).split();

    tokio::spawn(async move {
        while let Some(item) = outbound_reader.recv().await {
            outbound.send(item).await?;
        }

        io::Result::Ok(())
    });

    tokio::spawn(async move {
        while let Some(Ok(item)) = inbound.next().await {
            inbound_writer.send(item).unwrap();
        }

        io::Result::Ok(())
    });

    Ok(())
}
