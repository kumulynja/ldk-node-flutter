#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.72.1.

use crate::ldk::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_new__static_method__BuilderBase_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new__static_method__BuilderBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(BuilderBase::new()),
    )
}
fn wire_set_entropy_seed_path__method__BuilderBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<BuilderBase> + UnwindSafe,
    seed_path: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_entropy_seed_path__method__BuilderBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_seed_path = seed_path.wire2api();
            move |task_callback| Ok(BuilderBase::set_entropy_seed_path(&api_that, api_seed_path))
        },
    )
}
fn wire_set_entropy_seed_bytes__method__BuilderBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<BuilderBase> + UnwindSafe,
    seed_bytes: impl Wire2Api<[u8; 64]> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_entropy_seed_bytes__method__BuilderBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_seed_bytes = seed_bytes.wire2api();
            move |task_callback| {
                Ok(BuilderBase::set_entropy_seed_bytes(
                    &api_that,
                    api_seed_bytes,
                ))
            }
        },
    )
}
fn wire_set_storage_dir_path__method__BuilderBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<BuilderBase> + UnwindSafe,
    storage_dir_path: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_storage_dir_path__method__BuilderBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_storage_dir_path = storage_dir_path.wire2api();
            move |task_callback| {
                Ok(BuilderBase::set_storage_dir_path(
                    &api_that,
                    api_storage_dir_path,
                ))
            }
        },
    )
}
fn wire_set_esplora_server_url__method__BuilderBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<BuilderBase> + UnwindSafe,
    esplora_server_url: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_esplora_server_url__method__BuilderBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_esplora_server_url = esplora_server_url.wire2api();
            move |task_callback| {
                Ok(BuilderBase::set_esplora_server_url(
                    &api_that,
                    api_esplora_server_url,
                ))
            }
        },
    )
}
fn wire_set_network__method__BuilderBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<BuilderBase> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_network__method__BuilderBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_network = network.wire2api();
            move |task_callback| Ok(BuilderBase::set_network(&api_that, api_network))
        },
    )
}
fn wire_set_listening_address__method__BuilderBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<BuilderBase> + UnwindSafe,
    listening_address: impl Wire2Api<SocketAddr> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_listening_address__method__BuilderBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_listening_address = listening_address.wire2api();
            move |task_callback| {
                Ok(BuilderBase::set_listening_address(
                    &api_that,
                    api_listening_address,
                ))
            }
        },
    )
}
fn wire_set_entropy_bip39_mnemonic__method__BuilderBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<BuilderBase> + UnwindSafe,
    mnemonic: impl Wire2Api<String> + UnwindSafe,
    passphrase: impl Wire2Api<Option<String>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_entropy_bip39_mnemonic__method__BuilderBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_mnemonic = mnemonic.wire2api();
            let api_passphrase = passphrase.wire2api();
            move |task_callback| {
                Ok(BuilderBase::set_entropy_bip39_mnemonic(
                    &api_that,
                    api_mnemonic,
                    api_passphrase,
                ))
            }
        },
    )
}
fn wire_build__static_method__BuilderBase_impl(
    port_: MessagePort,
    builder: impl Wire2Api<BuilderBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "build__static_method__BuilderBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_builder = builder.wire2api();
            move |task_callback| Ok(BuilderBase::build(api_builder))
        },
    )
}
fn wire_start__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "start__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| NodeBase::start(&api_that)
        },
    )
}
fn wire_stop__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "stop__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| NodeBase::stop(&api_that)
        },
    )
}
fn wire_event_handled__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "event_handled__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| NodeBase::event_handled(&api_that)
        },
    )
}
fn wire_next_event__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "next_event__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| NodeBase::next_event(&api_that)
        },
    )
}
fn wire_node_id__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "node_id__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| NodeBase::node_id(&api_that)
        },
    )
}
fn wire_listening_address__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "listening_address__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(NodeBase::listening_address(&api_that))
        },
    )
}
fn wire_new_funding_address__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_funding_address__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| NodeBase::new_funding_address(&api_that)
        },
    )
}
fn wire_on_chain_balance__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "on_chain_balance__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| NodeBase::on_chain_balance(&api_that)
        },
    )
}
fn wire_send_to_on_chain_address__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    address: impl Wire2Api<Address> + UnwindSafe,
    amount_sats: impl Wire2Api<u64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "send_to_on_chain_address__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_address = address.wire2api();
            let api_amount_sats = amount_sats.wire2api();
            move |task_callback| {
                NodeBase::send_to_on_chain_address(&api_that, api_address, api_amount_sats)
            }
        },
    )
}
fn wire_send_all_to_on_chain_address__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    address: impl Wire2Api<Address> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "send_all_to_on_chain_address__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_address = address.wire2api();
            move |task_callback| NodeBase::send_all_to_on_chain_address(&api_that, api_address)
        },
    )
}
fn wire_connect__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    node_id: impl Wire2Api<PublicKey> + UnwindSafe,
    address: impl Wire2Api<SocketAddr> + UnwindSafe,
    permanently: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "connect__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_node_id = node_id.wire2api();
            let api_address = address.wire2api();
            let api_permanently = permanently.wire2api();
            move |task_callback| {
                NodeBase::connect(&api_that, api_node_id, api_address, api_permanently)
            }
        },
    )
}
fn wire_disconnect__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    counterparty_node_id: impl Wire2Api<PublicKey> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "disconnect__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_counterparty_node_id = counterparty_node_id.wire2api();
            move |task_callback| NodeBase::disconnect(&api_that, api_counterparty_node_id)
        },
    )
}
fn wire_connect_open_channel__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    address: impl Wire2Api<SocketAddr> + UnwindSafe,
    node_id: impl Wire2Api<PublicKey> + UnwindSafe,
    channel_amount_sats: impl Wire2Api<u64> + UnwindSafe,
    push_to_counterparty_msat: impl Wire2Api<Option<u64>> + UnwindSafe,
    announce_channel: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "connect_open_channel__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_address = address.wire2api();
            let api_node_id = node_id.wire2api();
            let api_channel_amount_sats = channel_amount_sats.wire2api();
            let api_push_to_counterparty_msat = push_to_counterparty_msat.wire2api();
            let api_announce_channel = announce_channel.wire2api();
            move |task_callback| {
                NodeBase::connect_open_channel(
                    &api_that,
                    api_address,
                    api_node_id,
                    api_channel_amount_sats,
                    api_push_to_counterparty_msat,
                    api_announce_channel,
                )
            }
        },
    )
}
fn wire_list_channels__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "list_channels__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(NodeBase::list_channels(&api_that))
        },
    )
}
fn wire_sync_wallets__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "sync_wallets__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| NodeBase::sync_wallets(&api_that)
        },
    )
}
fn wire_close_channel__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    channel_id: impl Wire2Api<[u8; 32]> + UnwindSafe,
    counterparty_node_id: impl Wire2Api<PublicKey> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "close_channel__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_channel_id = channel_id.wire2api();
            let api_counterparty_node_id = counterparty_node_id.wire2api();
            move |task_callback| {
                NodeBase::close_channel(&api_that, api_channel_id, api_counterparty_node_id)
            }
        },
    )
}
fn wire_send_payment__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    invoice: impl Wire2Api<Invoice> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "send_payment__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_invoice = invoice.wire2api();
            move |task_callback| NodeBase::send_payment(&api_that, api_invoice)
        },
    )
}
fn wire_send_payment_using_amount__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    invoice: impl Wire2Api<Invoice> + UnwindSafe,
    amount_msat: impl Wire2Api<u64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "send_payment_using_amount__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_invoice = invoice.wire2api();
            let api_amount_msat = amount_msat.wire2api();
            move |task_callback| {
                NodeBase::send_payment_using_amount(&api_that, api_invoice, api_amount_msat)
            }
        },
    )
}
fn wire_send_spontaneous_payment__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    amount_msat: impl Wire2Api<u64> + UnwindSafe,
    node_id: impl Wire2Api<PublicKey> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "send_spontaneous_payment__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_amount_msat = amount_msat.wire2api();
            let api_node_id = node_id.wire2api();
            move |task_callback| {
                NodeBase::send_spontaneous_payment(&api_that, api_amount_msat, api_node_id)
            }
        },
    )
}
fn wire_receive_payment__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    amount_msat: impl Wire2Api<u64> + UnwindSafe,
    description: impl Wire2Api<String> + UnwindSafe,
    expiry_secs: impl Wire2Api<u32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "receive_payment__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_amount_msat = amount_msat.wire2api();
            let api_description = description.wire2api();
            let api_expiry_secs = expiry_secs.wire2api();
            move |task_callback| {
                NodeBase::receive_payment(
                    &api_that,
                    api_amount_msat,
                    api_description,
                    api_expiry_secs,
                )
            }
        },
    )
}
fn wire_receive_variable_amount_payment__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    description: impl Wire2Api<String> + UnwindSafe,
    expiry_secs: impl Wire2Api<u32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "receive_variable_amount_payment__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_description = description.wire2api();
            let api_expiry_secs = expiry_secs.wire2api();
            move |task_callback| {
                NodeBase::receive_variable_amount_payment(
                    &api_that,
                    api_description,
                    api_expiry_secs,
                )
            }
        },
    )
}
fn wire_payment__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    payment_hash: impl Wire2Api<PaymentHash> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "payment__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_payment_hash = payment_hash.wire2api();
            move |task_callback| Ok(NodeBase::payment(&api_that, api_payment_hash))
        },
    )
}
fn wire_remove_payment__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    payment_hash: impl Wire2Api<PaymentHash> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "remove_payment__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_payment_hash = payment_hash.wire2api();
            move |task_callback| NodeBase::remove_payment(&api_that, api_payment_hash)
        },
    )
}
fn wire_list_payments_with_filter__method__NodeBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<NodeBase> + UnwindSafe,
    payment_direction: impl Wire2Api<PaymentDirection> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "list_payments_with_filter__method__NodeBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_payment_direction = payment_direction.wire2api();
            move |task_callback| {
                Ok(NodeBase::list_payments_with_filter(
                    &api_that,
                    api_payment_direction,
                ))
            }
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<Network> for i32 {
    fn wire2api(self) -> Network {
        match self {
            0 => Network::Bitcoin,
            1 => Network::Testnet,
            2 => Network::Signet,
            3 => Network::Regtest,
            _ => unreachable!("Invalid variant for Network: {}", self),
        }
    }
}

impl Wire2Api<PaymentDirection> for i32 {
    fn wire2api(self) -> PaymentDirection {
        match self {
            0 => PaymentDirection::Inbound,
            1 => PaymentDirection::Outbound,
            _ => unreachable!("Invalid variant for PaymentDirection: {}", self),
        }
    }
}

impl Wire2Api<u16> for u16 {
    fn wire2api(self) -> u16 {
        self
    }
}
impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for Address {
    fn into_dart(self) -> support::DartAbi {
        vec![self.address_hex.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Address {}

impl support::IntoDart for Balance {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.immature.into_dart(),
            self.trusted_pending.into_dart(),
            self.untrusted_pending.into_dart(),
            self.confirmed.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Balance {}

impl support::IntoDart for BuilderBase {
    fn into_dart(self) -> support::DartAbi {
        vec![self.config.into_dart(), self.entropy_source.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for BuilderBase {}

impl support::IntoDart for ChannelDetails {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.channel_id.into_dart(),
            self.funding_txo.into_dart(),
            self.short_channel_id.into_dart(),
            self.outbound_scid_alias.into_dart(),
            self.inbound_scid_alias.into_dart(),
            self.channel_value_satoshis.into_dart(),
            self.unspendable_punishment_reserve.into_dart(),
            self.user_channel_id.into_dart(),
            self.feerate_sat_per_1000_weight.into_dart(),
            self.balance_msat.into_dart(),
            self.outbound_capacity_msat.into_dart(),
            self.next_outbound_htlc_limit_msat.into_dart(),
            self.inbound_capacity_msat.into_dart(),
            self.confirmations_required.into_dart(),
            self.confirmations.into_dart(),
            self.force_close_spend_delay.into_dart(),
            self.is_outbound.into_dart(),
            self.is_channel_ready.into_dart(),
            self.is_usable.into_dart(),
            self.is_public.into_dart(),
            self.inbound_htlc_minimum_msat.into_dart(),
            self.inbound_htlc_maximum_msat.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ChannelDetails {}

impl support::IntoDart for Config {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.storage_dir_path.into_dart(),
            self.esplora_server_url.into_dart(),
            self.network.into_dart(),
            self.listening_address.into_dart(),
            self.default_cltv_expiry_delta.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Config {}

impl support::IntoDart for Event {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::PaymentSuccessful { payment_hash } => {
                vec![0.into_dart(), payment_hash.into_dart()]
            }
            Self::PaymentFailed { payment_hash } => vec![1.into_dart(), payment_hash.into_dart()],
            Self::PaymentReceived {
                payment_hash,
                amount_msat,
            } => vec![
                2.into_dart(),
                payment_hash.into_dart(),
                amount_msat.into_dart(),
            ],
            Self::ChannelReady {
                channel_id,
                user_channel_id,
            } => vec![
                3.into_dart(),
                channel_id.into_dart(),
                user_channel_id.into_dart(),
            ],
            Self::ChannelClosed {
                channel_id,
                user_channel_id,
            } => vec![
                4.into_dart(),
                channel_id.into_dart(),
                user_channel_id.into_dart(),
            ],
            Self::ChannelPending {
                channel_id,
                user_channel_id,
                former_temporary_channel_id,
                counterparty_node_id,
                funding_txo,
            } => vec![
                5.into_dart(),
                channel_id.into_dart(),
                user_channel_id.into_dart(),
                former_temporary_channel_id.into_dart(),
                counterparty_node_id.into_dart(),
                funding_txo.into_dart(),
            ],
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Event {}

impl support::IntoDart for Invoice {
    fn into_dart(self) -> support::DartAbi {
        vec![self.hex.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Invoice {}

impl support::IntoDart for Network {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Bitcoin => 0,
            Self::Testnet => 1,
            Self::Signet => 2,
            Self::Regtest => 3,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Network {}
impl support::IntoDart for NodeBase {
    fn into_dart(self) -> support::DartAbi {
        vec![self.node_pointer.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for NodeBase {}

impl support::IntoDart for OutPoint {
    fn into_dart(self) -> support::DartAbi {
        vec![self.txid.into_dart(), self.vout.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for OutPoint {}

impl support::IntoDart for PaymentDetails {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.hash.into_dart(),
            self.preimage.into_dart(),
            self.secret.into_dart(),
            self.amount_msat.into_dart(),
            self.direction.into_dart(),
            self.status.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PaymentDetails {}

impl support::IntoDart for PaymentDirection {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Inbound => 0,
            Self::Outbound => 1,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PaymentDirection {}
impl support::IntoDart for PaymentHash {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PaymentHash {}

impl support::IntoDart for PaymentPreimage {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PaymentPreimage {}

impl support::IntoDart for PaymentSecret {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PaymentSecret {}

impl support::IntoDart for PaymentStatus {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Pending => 0,
            Self::Succeeded => 1,
            Self::Failed => 2,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PaymentStatus {}
impl support::IntoDart for PublicKey {
    fn into_dart(self) -> support::DartAbi {
        vec![self.key_hex.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PublicKey {}

impl support::IntoDart for SocketAddr {
    fn into_dart(self) -> support::DartAbi {
        vec![self.ip.into_dart(), self.port.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for SocketAddr {}

impl support::IntoDart for Txid {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Txid {}

impl support::IntoDart for WalletEntropySource {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::SeedFile(field0) => vec![0.into_dart(), field0.into_dart()],
            Self::SeedBytes(field0) => vec![1.into_dart(), field0.into_dart()],
            Self::Bip39Mnemonic {
                mnemonic,
                passphrase,
            } => vec![2.into_dart(), mnemonic.into_dart(), passphrase.into_dart()],
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for WalletEntropySource {}
// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
