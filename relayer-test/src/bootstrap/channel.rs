use eyre::{eyre, Report as Error};
use ibc::core::ics04_channel::channel::Order;
use ibc::core::ics24_host::identifier::PortId;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::channel::Channel as BaseChannel;
use ibc_relayer::config::default;
use ibc_relayer::connection::Connection;
use ibc_relayer::foreign_client::ForeignClient;

use crate::tagged::*;
use crate::types::binary::chains::ConnectedChains;
use crate::types::binary::channel::Channel;

pub fn bootstrap_channel_with_chains<ChainA: ChainHandle, ChainB: ChainHandle>(
    chains: &ConnectedChains<ChainA, ChainB>,
    port_a: &PortId,
    port_b: &PortId,
) -> Result<Channel<ChainA, ChainB>, Error> {
    let channel = bootstrap_channel(
        &chains.client_b_to_a,
        &chains.client_a_to_b,
        &DualTagged::new(port_a),
        &DualTagged::new(port_b),
    )?;

    Ok(channel)
}

// Create a new connected channel between chain A and B
// using new IBC client and connection.
pub fn bootstrap_channel<ChainA: ChainHandle, ChainB: ChainHandle>(
    client_b_to_a: &ForeignClient<ChainA, ChainB>,
    client_a_to_b: &ForeignClient<ChainB, ChainA>,
    port_a: &DualTagged<ChainA, ChainB, &PortId>,
    port_b: &DualTagged<ChainB, ChainA, &PortId>,
) -> Result<Channel<ChainA, ChainB>, Error> {
    let connection = Connection::new(
        client_b_to_a.clone(),
        client_a_to_b.clone(),
        default::connection_delay(),
    )?;

    let channel = BaseChannel::new(
        connection,
        Order::Unordered,
        port_a.0.clone(),
        port_b.0.clone(),
        None,
    )?;

    let channel_id_a = channel
        .a_side
        .channel_id()
        .ok_or_else(|| eyre!("expect channel id"))?
        .clone();

    let channel_id_b = channel
        .b_side
        .channel_id()
        .ok_or_else(|| eyre!("expect channel id"))?
        .clone();

    let res = Channel {
        channel,
        channel_id_a: DualTagged::new(channel_id_a),
        channel_id_b: DualTagged::new(channel_id_b),
        port_a: port_a.cloned(),
        port_b: port_b.cloned(),
    };

    Ok(res)
}
