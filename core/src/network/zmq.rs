use crate::pipeline::construct_gdp_advertisement_from_bytes;
use crate::pipeline::proc_gdp_packet;

use crate::structs::GDPName;
use crate::structs::{GDPChannel, GDPPacket, GdpAction, Packet};
use std::io;

use std::{net::SocketAddr, str::FromStr};
use futures::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
const UDP_BUFFER_SIZE: usize = 17480; // 17480 17kb TODO: make it formal
use crate::pipeline::construct_gdp_forward_from_bytes;
use crate::structs::GDPPacketInTransit;
use rand::Rng;

fn generate_random_gdp_name_for_thread() -> GDPName {
    // u8:4
    GDPName([
        rand::thread_rng().gen(),
        rand::thread_rng().gen(),
        rand::thread_rng().gen(),
        rand::thread_rng().gen(),
    ])
}

use futures::StreamExt;

use log::info;

use std::env;
use tmq::{pull, push, Context, Result};



pub async fn zmq_router(
    ip_address: &str, 
    port: &str
) -> Result<()> {
    let mut socket_pull = pull(&Context::new()).bind(format!("{}:{}", ip_address, port).as_str())?;
    let mut socket_push = push(&Context::new()).connect(format!("{}:{}", ip_address, port).as_str())?;

    while let Some(msg) = socket_pull.next().await {
        let msg = msg.unwrap();
        info!(
            "Pull: {:?}", msg.iter()
            .map(|item| item.as_str().unwrap_or("invalid text"))
            .collect::<Vec<&str>>()
        );
        socket_push.send(msg).await?;
    }
    Ok(())
}


// /// does not go to rib when peering
// pub async fn zmq_to_peer_direct(
//     addr: String,
//     rib_tx: UnboundedSender<GDPPacket>,
//     channel_tx: UnboundedSender<GDPChannel>,
//     peer_tx: UnboundedSender<GDPPacket>,   // used
//     peer_rx: UnboundedReceiver<GDPPacket>, // used to send packet over the network
// ) {
//     let stream = match TcpStream::connect(SocketAddr::from_str(&addr).unwrap()).await {
//         Ok(s) => s,
//         Err(_) => {
//             error!("TCP: Unable to connect to the dafault gateway {}", addr);
//             return;
//         }
//     };

//     println!("{:?}", stream);

//     let m_gdp_name = generate_random_gdp_name_for_thread();
//     info!("TCP connection takes gdp name {:?}", m_gdp_name);

//     handle_tcp_stream(stream, &rib_tx, &channel_tx, peer_tx, peer_rx, m_gdp_name).await;
// }
