use crate::api::error::{BuilderException, NodeException};
use crate::api::node::LdkMnemonic;
use flutter_rust_bridge::frb;
use std::str::FromStr;
use std::string::ToString;

///An address which can be used to connect to a remote peer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketAddress {
    TcpIpV4 {
        addr: [u8; 4],
        port: u16,
    },
    TcpIpV6 {
        addr: [u8; 16],
        port: u16,
    },
    OnionV2([u8; 12]),
    OnionV3 {
        ed25519_pubkey: [u8; 32],
        checksum: u16,
        version: u8,
        port: u16,
    },
    Hostname {
        addr: String,
        port: u16,
    },
}
impl SocketAddress {
    pub fn from_str(address: String) -> Result<SocketAddress, BuilderException> {
        match ldk_node::lightning::ln::msgs::SocketAddress::from_str(address.as_str()) {
            Ok(e) => Ok(e.into()),
            Err(_) => Err(BuilderException::SocketAddressParseError),
        }
    }
    pub fn as_string(&self) -> String {
        format!("{:?}", { self })
    }
}
impl From<ldk_node::lightning::ln::msgs::SocketAddress> for SocketAddress {
    fn from(value: ldk_node::lightning::ln::msgs::SocketAddress) -> Self {
        match value {
            ldk_node::lightning::ln::msgs::SocketAddress::TcpIpV4 { addr, port } => {
                SocketAddress::TcpIpV4 { addr, port }
            }
            ldk_node::lightning::ln::msgs::SocketAddress::TcpIpV6 { addr, port } => {
                SocketAddress::TcpIpV6 { addr, port }
            }
            ldk_node::lightning::ln::msgs::SocketAddress::OnionV2(e) => SocketAddress::OnionV2(e),
            ldk_node::lightning::ln::msgs::SocketAddress::OnionV3 {
                ed25519_pubkey,
                checksum,
                version,
                port,
            } => SocketAddress::OnionV3 {
                ed25519_pubkey,
                checksum,
                version,
                port,
            },
            ldk_node::lightning::ln::msgs::SocketAddress::Hostname { hostname, port } => {
                SocketAddress::Hostname {
                    addr: hostname.to_string(),
                    port,
                }
            }
        }
    }
}

impl TryFrom<SocketAddress> for ldk_node::lightning::ln::msgs::SocketAddress {
    type Error = BuilderException;

    fn try_from(value: SocketAddress) -> Result<Self, Self::Error> {
        match value {
            SocketAddress::TcpIpV4 { addr, port } => {
                Ok(ldk_node::lightning::ln::msgs::SocketAddress::TcpIpV4 { addr, port })
            }
            SocketAddress::TcpIpV6 { addr, port } => {
                Ok(ldk_node::lightning::ln::msgs::SocketAddress::TcpIpV6 { addr, port })
            }
            SocketAddress::OnionV2(e) => {
                Ok(ldk_node::lightning::ln::msgs::SocketAddress::OnionV2(e))
            }
            SocketAddress::OnionV3 {
                ed25519_pubkey,
                checksum,
                version,
                port,
            } => Ok(ldk_node::lightning::ln::msgs::SocketAddress::OnionV3 {
                ed25519_pubkey,
                checksum,
                version,
                port,
            }),
            SocketAddress::Hostname { addr, port } => {
                Ok(ldk_node::lightning::ln::msgs::SocketAddress::Hostname {
                    hostname: ldk_node::lightning::util::ser::Hostname::try_from(addr)
                        .map_err(|_| BuilderException::SocketAddressParseError)?,
                    port,
                })
            }
        }
    }
}
///Options which apply on a per-channel basis and may change at runtime or based on negotiation with our counterparty.
pub struct ChannelConfig {
    ///Amount (in millionths of a satoshi) charged per satoshi for payments forwarded outbound over the channel. This may be allowed to change at runtime in a later update, however doing so must result in update messages sent to notify all nodes of our updated relay fee.
    ///
    ///Default value: 0.
    pub forwarding_fee_proportional_millionths: u32,
    /// Amount (in milli-satoshi) charged for payments forwarded outbound over the channel, in excess of forwardingFeeProportionalMillionths. This may be allowed to change at runtime in a later update, however doing so must result in update messages sent to notify all nodes of our updated relay fee.
    ///
    /// The default value of a single satoshi roughly matches the market rate on many routing nodes as of July 2021. Adjusting it upwards or downwards may change whether nodes route through this node.
    ///
    ///Default value: 1000.
    pub forwarding_fee_base_msat: u32,
    ///The difference in the CLTV value between incoming HTLCs and an outbound HTLC forwarded over the channel this config applies to.
    ///
    /// Thus, for HTLC-encumbered balances to be enforced on-chain when a channel is force-closed, we (or one of our watchtowers) MUST be online to check for broadcast of the current commitment transaction at least once per this many blocks (minus some margin to allow us enough time to broadcast and confirm a transaction, possibly with time in between to RBF the spending transaction).
    ///
    /// Default value: 72 (12 hours at an average of 6 blocks/hour). Minimum value: MIN_CLTV_EXPIRY_DELTA, any values less than this will be treated as MIN_CLTV_EXPIRY_DELTA instead.
    pub cltv_expiry_delta: u16,
    /// Options for how to set the max dust HTLC exposure allowed on a channel.
    pub max_dust_htlc_exposure: MaxDustHTLCExposure,
    ///The additional fee we’re willing to pay to avoid waiting for the counterparty’s toSelfDelay to reclaim funds.
    ///
    /// When we close a channel cooperatively with our counterparty, we negotiate a fee for the closing transaction which both sides find acceptable, ultimately paid by the channel funder/initiator.
    ///
    /// When we are the funder, because we have to pay the channel closing fee, we bound the acceptable fee by our Background and Normal fees, with the upper bound increased by this value. Because the on-chain fee we’d pay to force-close the channel is kept near our Normal feerate during normal operation, this value represents the additional fee we’re willing to pay in order to avoid waiting for our counterparty’s toSelfDelay to reclaim our funds.
    ///
    /// When we are not the funder, we require the closing transaction fee pay at least our Background fee estimate, but allow our counterparty to pay as much fee as they like. Thus, this value is ignored when we are not the funder.
    ///
    /// Default value: 1000 satoshis.
    pub force_close_avoidance_max_fee_satoshis: u64,
    ///If set, allows this channel's counterparty to skim an additional fee off this node's inbound HTLCs. Useful for liquidity providers to offload on-chain channel costs to end users.
    /// Usage:
    /// The payee will set this option and set its invoice route hints to use intercept scids generated by this channel's counterparty.
    pub accept_underpaying_htlcs: bool,
}
pub enum MaxDustHTLCExposure {
    ///This sets a fixed limit on the total dust exposure in millisatoshis. Setting this too low may prevent the sending or receipt of low-value HTLCs on high-traffic nodes, however this limit is very important to prevent stealing of large amounts of dust HTLCs by miners through fee griefing attacks.
    //
    // Note that if the feerate increases significantly, without a manual increase to this maximum the channel may be unable to send/receive HTLCs between the maximum dust exposure and the new minimum value for HTLCs to be economically viable to claim.
    FixedLimitMsat(u64),
    ///This sets a multiplier on the estimated high priority feerate (sats/KW, as obtained from FeeEstimator) to determine the maximum allowed dust exposure. If this variant is used then the maximum dust exposure in millisatoshis is calculated as: high_priority_feerate_per_kw * value. For example, with our default value FeeRateMultiplier(5000):
    //
    // For the minimum fee rate of 1 sat/vByte (250 sat/KW, although the minimum defaults to 253 sats/KW for rounding, see FeeEstimator), the max dust exposure would be 253 * 5000 = 1,265,000 msats.
    // For a fee rate of 30 sat/vByte (7500 sat/KW), the max dust exposure would be 7500 * 5000 = 37,500,000 msats.
    // This allows the maximum dust exposure to automatically scale with fee rate changes.
    //
    // Note, if you’re using a third-party fee estimator, this may leave you more exposed to a fee griefing attack, where your fee estimator may purposely overestimate the fee rate, causing you to accept more dust HTLCs than you would otherwise.
    //
    // This variant is primarily meant to serve pre-anchor channels, as HTLC fees being included on HTLC outputs means your channel may be subject to more dust exposure in the event of increases in fee rate.
    FeeRateMultiplier(u64),
}
impl From<MaxDustHTLCExposure> for ldk_node::lightning::util::config::MaxDustHTLCExposure {
    fn from(value: MaxDustHTLCExposure) -> Self {
        match value {
            MaxDustHTLCExposure::FixedLimitMsat(e) => {
                ldk_node::lightning::util::config::MaxDustHTLCExposure::FixedLimitMsat(e)
            }
            MaxDustHTLCExposure::FeeRateMultiplier(e) => {
                ldk_node::lightning::util::config::MaxDustHTLCExposure::FeeRateMultiplier(e)
            }
        }
    }
}
impl From<ChannelConfig> for ldk_node::ChannelConfig {
    fn from(x: ChannelConfig) -> Self {
        let config =
            ldk_node::ChannelConfig::from(ldk_node::lightning::util::config::ChannelConfig {
                forwarding_fee_proportional_millionths: x.forwarding_fee_proportional_millionths,
                forwarding_fee_base_msat: x.forwarding_fee_base_msat,
                cltv_expiry_delta: x.cltv_expiry_delta,
                max_dust_htlc_exposure: x.max_dust_htlc_exposure.into(),
                force_close_avoidance_max_fee_satoshis: x.force_close_avoidance_max_fee_satoshis,
                accept_underpaying_htlcs: x.accept_underpaying_htlcs,
            });
        config
    }
}
/// The global identifier of a channel.
///
/// Note that this will start out to be a temporary ID until channel funding negotiation is
/// finalized, at which point it will change to be a permanent global ID tied to the on-chain
/// funding transaction.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelId {
    pub data: [u8; 32],
}

impl From<ldk_node::lightning::ln::ChannelId> for ChannelId {
    fn from(value: ldk_node::lightning::ln::ChannelId) -> Self {
        ChannelId { data: value.0 }
    }
}
impl From<ChannelId> for ldk_node::lightning::ln::ChannelId {
    fn from(value: ChannelId) -> Self {
        ldk_node::lightning::ln::ChannelId(value.data)
    }
}
///A local, potentially user-provided, identifier of a channel.
///
/// By default, this will be randomly generated for the user to ensure local uniqueness.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserChannelId {
    pub data: u64,
}
/// An event emitted by [Node], which should be handled by the user.
///
impl From<ldk_node::UserChannelId> for UserChannelId {
    fn from(value: ldk_node::UserChannelId) -> Self {
        UserChannelId {
            data: value.0 as u64,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// An event emitted by [Node], which should be handled by the user.
///
pub enum Event {
    /// A sent payment was successful.
    PaymentSuccessful {
        /// The hash of the payment.
        payment_hash: PaymentHash,
    },
    /// A sent payment has failed.
    PaymentFailed {
        /// The hash of the payment.
        payment_hash: PaymentHash,
    },
    /// A payment has been received.
    PaymentReceived {
        /// The hash of the payment.
        payment_hash: PaymentHash,
        /// The value, in thousandths of a satoshi, that has been received.
        amount_msat: u64,
    },
    /// A channel is ready to be used.
    ChannelReady {
        /// The channel_id of the channel.
        channel_id: ChannelId,
        /// The user_channel_id of the channel.
        user_channel_id: UserChannelId,

        counterparty_node_id: Option<PublicKey>,
    },
    /// A channel has been closed.
    ChannelClosed {
        /// The channel_id of the channel.
        channel_id: ChannelId,
        /// The user_channel_id of the channel.
        user_channel_id: UserChannelId,
        counterparty_node_id: Option<PublicKey>,
    },
    /// A channel has been created and is pending confirmation on-chain.
    ChannelPending {
        /// The channel_id of the channel.
        channel_id: ChannelId,
        /// The user_channel_id of the channel.
        user_channel_id: UserChannelId,
        /// The temporary_channel_id this channel used to be known by during channel establishment.
        former_temporary_channel_id: ChannelId,
        /// The node_id of the channel counterparty.
        counterparty_node_id: PublicKey,
        /// The outpoint of the channel's funding transaction.
        funding_txo: OutPoint,
    },
}

impl From<ldk_node::Event> for Event {
    fn from(value: ldk_node::Event) -> Self {
        match value {
            ldk_node::Event::PaymentSuccessful { payment_hash } => Event::PaymentSuccessful {
                payment_hash: PaymentHash {
                    data: payment_hash.0,
                },
            },
            ldk_node::Event::PaymentFailed { payment_hash } => Event::PaymentFailed {
                payment_hash: PaymentHash {
                    data: payment_hash.0,
                },
            },
            ldk_node::Event::PaymentReceived {
                payment_hash,
                amount_msat,
            } => Event::PaymentReceived {
                payment_hash: PaymentHash {
                    data: payment_hash.0,
                },
                amount_msat,
            },
            ldk_node::Event::ChannelReady {
                channel_id,
                user_channel_id,
                counterparty_node_id,
            } => Event::ChannelReady {
                channel_id: channel_id.into(),
                user_channel_id: user_channel_id.into(),
                counterparty_node_id: counterparty_node_id.map(|x| x.into()),
            },
            ldk_node::Event::ChannelClosed {
                channel_id,
                user_channel_id,
                counterparty_node_id,
            } => Event::ChannelClosed {
                channel_id: channel_id.into(),
                user_channel_id: user_channel_id.into(),
                counterparty_node_id: counterparty_node_id.map(|x| x.into()),
            },
            ldk_node::Event::ChannelPending {
                channel_id,
                user_channel_id,
                former_temporary_channel_id,
                counterparty_node_id,
                funding_txo,
            } => Event::ChannelPending {
                channel_id: channel_id.into(),
                user_channel_id: user_channel_id.into(),
                former_temporary_channel_id: former_temporary_channel_id.into(),
                counterparty_node_id: PublicKey {
                    hex: counterparty_node_id.to_string(),
                },
                funding_txo: funding_txo.into(),
            },
        }
    }
}

///A bitcoin transaction hash/transaction ID.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Txid {
    pub hash: String,
}

impl TryFrom<Txid> for ldk_node::bitcoin::Txid {
    type Error = NodeException;

    fn try_from(value: Txid) -> Result<Self, Self::Error> {
        ldk_node::bitcoin::Txid::from_str(value.hash.as_str())
            .map_err(|_| NodeException::InvalidTxid)
    }
}

impl From<ldk_node::bitcoin::Txid> for Txid {
    fn from(value: ldk_node::bitcoin::Txid) -> Self {
        Txid {
            hash: value.to_string(),
        }
    }
}
///A reference to a transaction output.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl From<ldk_node::bitcoin::OutPoint> for OutPoint {
    fn from(value: ldk_node::bitcoin::OutPoint) -> Self {
        OutPoint {
            txid: Txid {
                hash: value.txid.to_raw_hash().to_string(),
            },
            vout: value.vout,
        }
    }
}

/// Represents the current status of a payment.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PaymentStatus {
    /// The payment is still pending.
    Pending,
    /// The payment suceeded.
    Succeeded,
    /// The payment failed.
    Failed,
}

impl From<ldk_node::PaymentStatus> for PaymentStatus {
    fn from(value: ldk_node::PaymentStatus) -> Self {
        match value {
            ldk_node::PaymentStatus::Pending => PaymentStatus::Pending,
            ldk_node::PaymentStatus::Succeeded => PaymentStatus::Succeeded,
            ldk_node::PaymentStatus::Failed => PaymentStatus::Failed,
        }
    }
}

/// Represents the direction of a payment.
///
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PaymentDirection {
    /// The payment is inbound.
    Inbound,
    /// The payment is outbound.
    Outbound,
}

impl From<ldk_node::PaymentDirection> for PaymentDirection {
    fn from(value: ldk_node::PaymentDirection) -> Self {
        match value {
            ldk_node::PaymentDirection::Inbound => PaymentDirection::Inbound,
            ldk_node::PaymentDirection::Outbound => PaymentDirection::Outbound,
        }
    }
}

impl From<PaymentDirection> for ldk_node::PaymentDirection {
    fn from(value: PaymentDirection) -> Self {
        match value {
            PaymentDirection::Inbound => ldk_node::PaymentDirection::Inbound,
            PaymentDirection::Outbound => ldk_node::PaymentDirection::Outbound,
        }
    }
}

/// paymentHash type, use to cross-lock hop
///
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PaymentHash {
    pub data: [u8; 32],
}

impl From<PaymentHash> for ldk_node::lightning::ln::PaymentHash {
    fn from(value: PaymentHash) -> Self {
        ldk_node::lightning::ln::PaymentHash(value.data)
    }
}

impl From<ldk_node::lightning::ln::PaymentHash> for PaymentHash {
    fn from(value: ldk_node::lightning::ln::PaymentHash) -> Self {
        PaymentHash { data: value.0 }
    }
}

/// paymentPreimage type, use to route payment between hop
///
#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub struct PaymentPreimage {
    pub data: [u8; 32],
}

/// payment_secret type, use to authenticate sender to the receiver and tie MPP HTLCs together
///
#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub struct PaymentSecret {
    pub data: [u8; 32],
}

// Structs wrapping the particular information which should easily be
// understandable, parseable, and transformable, i.e., we'll try to avoid
// exposing too many technical detail here.
/// Represents a payment.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentDetails {
    /// The payment hash, i.e., the hash of the `preimage`.
    pub hash: PaymentHash,
    /// The pre-image used by the payment.
    pub preimage: Option<PaymentPreimage>,
    /// The secret used by the payment.
    pub secret: Option<PaymentSecret>,
    /// The amount transferred.
    pub amount_msat: Option<u64>,
    /// The direction of the payment.
    pub direction: PaymentDirection,
    /// The status of the payment.
    pub status: PaymentStatus,
}

impl From<ldk_node::PaymentDetails> for PaymentDetails {
    fn from(value: ldk_node::PaymentDetails) -> Self {
        PaymentDetails {
            hash: PaymentHash { data: value.hash.0 },
            preimage: value.preimage.map(|x| PaymentPreimage { data: x.0 }),
            secret: value.secret.map(|x| PaymentSecret { data: x.0 }),
            status: value.status.into(),
            amount_msat: value.amount_msat,
            direction: value.direction.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
///Represents a syntactically and semantically correct lightning BOLT11 invoice.
///
pub struct Bolt11Invoice {
    pub signed_raw_invoice: String,
}

impl TryFrom<Bolt11Invoice> for ldk_node::lightning_invoice::Bolt11Invoice {
    type Error = NodeException;

    fn try_from(value: Bolt11Invoice) -> Result<Self, Self::Error> {
        ldk_node::lightning_invoice::Bolt11Invoice::from_str(value.signed_raw_invoice.as_str())
            .map_err(|_| NodeException::InvalidInvoice)
    }
}
impl From<ldk_node::lightning_invoice::Bolt11Invoice> for Bolt11Invoice {
    fn from(value: ldk_node::lightning_invoice::Bolt11Invoice) -> Self {
        Bolt11Invoice {
            signed_raw_invoice: value.to_string(),
        }
    }
}
///A Secp256k1 public key, used for verification of signatures.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    pub hex: String,
}

impl TryFrom<PublicKey> for ldk_node::bitcoin::secp256k1::PublicKey {
    type Error = NodeException;

    fn try_from(value: PublicKey) -> Result<Self, Self::Error> {
        ldk_node::bitcoin::secp256k1::PublicKey::from_str(value.hex.as_str())
            .map_err(|_| NodeException::InvalidPublicKey)
    }
}
impl From<ldk_node::bitcoin::secp256k1::PublicKey> for PublicKey {
    fn from(value: ldk_node::bitcoin::secp256k1::PublicKey) -> Self {
        PublicKey {
            hex: value.to_string(),
        }
    }
}
/// A Bitcoin address.
///
pub struct Address {
    pub s: String,
}

impl TryFrom<Address> for ldk_node::bitcoin::Address {
    type Error = NodeException;

    fn try_from(value: Address) -> Result<Self, Self::Error> {
        ldk_node::bitcoin::Address::from_str(value.s.as_str())
            .map(|e| e.assume_checked())
            .map_err(|_| NodeException::InvalidAddress)
    }
}
impl From<ldk_node::bitcoin::Address> for Address {
    fn from(value: ldk_node::bitcoin::Address) -> Self {
        Address {
            s: value.to_string(),
        }
    }
}

/// Details of a channel, as returned by node.listChannels()
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelDetails {
    /// The channel's ID (prior to funding transaction generation, this is a random 32 bytes,
    /// thereafter this is the txid of the funding transaction xor the funding transaction output).
    /// Note that this means this value is *not* persistent - it can change once during the
    /// lifetime of the channel.
    ///
    pub channel_id: ChannelId,
    ///The node ID of our the channel’s counterparty.
    pub counterparty_node_id: PublicKey,
    /// The Channel's funding transaction output, if we've negotiated the funding transaction with
    /// our counterparty already.
    ///
    pub funding_txo: Option<String>,
    ///The value, in satoshis, of this channel as it appears in the funding output.
    pub channel_value_sats: u64,
    /// The value, in satoshis, that must always be held in the channel for us. This value ensures
    /// that if we broadcast a revoked state, our counterparty can punish us by claiming at least
    /// this value on chain.
    ///
    /// This value is not included in `outbound_capacity_msat` as it can never be spent.
    ///
    /// This value will be null for outbound channels until the counterparty accepts the channel.
    ///
    pub unspendable_punishment_reserve: Option<u64>,
    /// The userChannelId passed in to create_channel, or a random value if the channel was
    /// inbound. This may be zero for inbound channels serialized with LDK versions prior to
    /// 0.0.113.
    ///
    pub user_channel_id: UserChannelId,
    /// The currently negotiated fee rate denominated in satoshi per 1000 weight units,
    /// which is applied to commitment and HTLC transactions.
    ///
    /// This value will be null for objects serialized with LDK versions prior to 0.0.115.
    ///
    pub feerate_sat_per_1000_weight: u32,
    /// Our total balance.  This is the amount we would get if we close the channel.
    /// This value is not exact. Due to various in-flight changes and feerate changes, exactly this
    /// amount is not likely to be recoverable on close.
    ///
    /// This does not include any pending HTLCs which are not yet fully resolved (and, thus, whose
    /// balance is not available for inclusion in new outbound HTLCs). This further does not include
    /// any pending outgoing HTLCs which are awaiting some other resolution to be sent.
    /// This does not consider any on-chain fees.
    ///
    pub balance_msat: u64,
    /// The available outbound capacity for sending HTLCs to the remote peer. This does not include
    /// any pending HTLCs which are not yet fully resolved (and, thus, whose balance is not
    /// available for inclusion in new outbound HTLCs). This further does not include any pending
    /// outgoing HTLCs which are awaiting some other resolution to be sent.
    ///
    /// This value is not exact. Due to various in-flight changes, feerate changes, and our
    /// conflict-avoidance policy, exactly this amount is not likely to be spendable. However, we
    /// should be able to spend nearly this amount.
    ///
    pub outbound_capacity_msat: u64,
    /// The available inbound capacity for the remote peer to send HTLCs to us. This does not
    /// include any pending HTLCs which are not yet fully resolved (and, thus, whose balance is not
    /// available for inclusion in new inbound HTLCs).
    /// Note that there are some corner cases not fully handled here, so the actual available
    /// inbound capacity may be slightly higher than this.
    ///
    /// This value is not exact. Due to various in-flight changes, feerate changes, and our
    /// counterparty's conflict-avoidance policy, exactly this amount is not likely to be spendable.
    /// However, our counterparty should be able to spend nearly this amount.
    ///
    pub inbound_capacity_msat: u64,
    /// The number of required confirmations on the funding transaction before the funding will be
    /// considered "locked". This number is selected by the channel fundee, and can be selected for inbound channels with
    /// This value will be null for outbound channels until the counterparty accepts the channel.
    ///
    pub confirmations_required: Option<u32>,
    /// The current number of confirmations on the funding transaction.
    ///
    /// This value will be `None` for objects serialized with LDK versions prior to 0.0.113.
    ///
    pub confirmations: Option<u32>,

    /// True if the channel was initiated (and thus funded) by us.
    ///
    pub is_outbound: bool,
    /// True if the channel is confirmed, channelReady messages have been exchanged, and the
    /// channel is not currently being shut down. `channelReady` message exchange implies the
    /// required confirmation count has been reached (and we were connected to the peer at some
    /// point after the funding transaction received enough confirmations). The required
    ///
    pub is_channel_ready: bool,
    /// True if the channel is (a) confirmed and channelReady messages have been exchanged, (b)
    /// the peer is connected, and (c) the channel is not currently negotiating a shutdown.
    ///
    /// This is a strict superset of `isChannelReady`.
    ///
    pub is_usable: bool,
    /// True if this channel is (or will be) publicly-announced.
    ///
    pub is_public: bool,
}
impl From<&ldk_node::ChannelDetails> for ChannelDetails {
    fn from(value: &ldk_node::ChannelDetails) -> Self {
        ChannelDetails {
            channel_id: value.clone().channel_id.into(),
            counterparty_node_id: value.clone().counterparty_node_id.into(),
            funding_txo: value.clone().funding_txo.map(|x| x.txid.to_string()),
            channel_value_sats: value.clone().channel_value_sats,
            unspendable_punishment_reserve: value.clone().unspendable_punishment_reserve,
            user_channel_id: value.clone().user_channel_id.into(),
            feerate_sat_per_1000_weight: value.clone().feerate_sat_per_1000_weight,
            balance_msat: value.clone().balance_msat,
            outbound_capacity_msat: value.clone().outbound_capacity_msat,
            inbound_capacity_msat: value.clone().inbound_capacity_msat,
            confirmations_required: value.clone().confirmations_required,
            confirmations: value.clone().confirmations,
            is_outbound: value.clone().is_outbound,
            is_channel_ready: value.clone().is_channel_ready,
            is_usable: value.clone().is_usable,
            is_public: value.clone().is_public,
        }
    }
}

/// Bitcoin network enum
///
#[derive(Debug, Clone)]
pub enum Network {
    ///Classic Bitcoin
    ///
    Bitcoin,
    ///Bitcoin’s testnet
    ///
    Testnet,
    ///Bitcoin’s signet
    ///
    Signet,
    ///Bitcoin’s regtest
    ///
    Regtest,
}

impl From<Network> for ldk_node::Network {
    fn from(value: Network) -> Self {
        match value {
            Network::Bitcoin => ldk_node::Network::Bitcoin,
            Network::Testnet => ldk_node::Network::Testnet,
            Network::Signet => ldk_node::Network::Signet,
            Network::Regtest => ldk_node::Network::Regtest,
        }
    }
}

/// Details of a known Lightning peer as returned by `node.listPeers`.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerDetails {
    /// Our peer's node ID.
    ///
    pub node_id: PublicKey,
    /// The IP address and TCP port of the peer.
    ///
    pub address: SocketAddress,
    /// Indicates whether or not the user is currently has an active connection with the peer.
    ///
    pub is_connected: bool,
}

impl From<ldk_node::PeerDetails> for PeerDetails {
    fn from(value: ldk_node::PeerDetails) -> Self {
        PeerDetails {
            node_id: value.node_id.into(),
            address: value.address.into(),
            is_connected: value.is_connected,
        }
    }
}

/// An enum representing the available verbosity levels of the logger.
///
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum LogLevel {
    /// Designates extremely verbose information, including gossip-induced messages
    ///
    Gossip,
    /// Designates very low priority, often extremely verbose, information
    ///
    Trace,
    /// Designates lower priority information
    ///
    Debug,
    /// Designates useful information
    ///
    Info,
    /// Designates hazardous situations
    ///
    Warn,
    /// Designates very serious errors
    ///
    Error,
}

impl From<LogLevel> for ldk_node::LogLevel {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Gossip => ldk_node::LogLevel::Gossip,
            LogLevel::Trace => ldk_node::LogLevel::Trace,
            LogLevel::Debug => ldk_node::LogLevel::Debug,
            LogLevel::Info => ldk_node::LogLevel::Info,
            LogLevel::Warn => ldk_node::LogLevel::Warn,
            LogLevel::Error => ldk_node::LogLevel::Error,
        }
    }
}

/// Represents the configuration of an [Node] instance.
///
#[frb]
#[derive(Debug, Clone)]
pub struct Config {
    #[frb(non_final)]
    pub storage_dir_path: String,
    #[frb(non_final)]
    pub log_dir_path: Option<String>,
    /// The used Bitcoin network.
    ///
    #[frb(non_final)]
    pub network: Network,
    /// The IP address and TCP port the node will listen on.
    ///
    #[frb(non_final)]
    pub listening_addresses: Option<Vec<SocketAddress>>,
    /// The default CLTV expiry delta to be used for payments.
    ///
    #[frb(non_final)]
    pub default_cltv_expiry_delta: u32,
    ///The time in-between background sync attempts of the onchain wallet, in seconds.
    /// Note: A minimum of 10 seconds is always enforced.

    #[frb(non_final)]
    pub onchain_wallet_sync_interval_secs: u64,

    /// The time in-between background sync attempts of the LDK wallet, in seconds.
    /// Note: A minimum of 10 seconds is always enforced.
    ///
    #[frb(non_final)]
    pub wallet_sync_interval_secs: u64,

    ///The time in-between background update attempts to our fee rate cache, in seconds.
    /// Note: A minimum of 10 seconds is always enforced.
    ///
    #[frb(non_final)]
    pub fee_rate_cache_update_interval_secs: u64,
    ///A list of peers that we allow to establish zero confirmation channels to us.
    ///
    ///Note: Allowing payments via zero-confirmation channels is potentially insecure if the funding transaction ends up never being confirmed on-chain. Zero-confirmation channels should therefore only be accepted from trusted peers.
    pub trusted_peers_0conf: Vec<PublicKey>,

    pub probing_liquidity_limit_multiplier: u64,
    ///The level at which we log messages.
    /// Any messages below this level will be excluded from the logs.
    ///
    #[frb(non_final)]
    pub log_level: LogLevel,
}
impl TryFrom<Config> for ldk_node::Config {
    type Error = BuilderException;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let addresses = if let Some(addresses) = value.listening_addresses {
            let addr_vec: Result<
                Vec<ldk_node::lightning::ln::msgs::SocketAddress>,
                BuilderException,
            > = addresses
                .into_iter()
                .map(|socket_addr| socket_addr.try_into())
                .collect();
            Some(addr_vec?)
        } else {
            None
        };
        let trusted_peers_0conf: Result<
            Vec<ldk_node::bitcoin::secp256k1::PublicKey>,
            BuilderException,
        > = value
            .trusted_peers_0conf
            .into_iter()
            .map(|x| {
                x.try_into()
                    .map_err(|_| BuilderException::InvalidTrustedPeer)
            })
            .collect();

        Ok(ldk_node::Config {
            storage_dir_path: value.storage_dir_path,
            log_dir_path: value.log_dir_path,
            network: value.network.into(),
            listening_addresses: addresses,
            default_cltv_expiry_delta: value.default_cltv_expiry_delta,
            onchain_wallet_sync_interval_secs: value.onchain_wallet_sync_interval_secs,
            wallet_sync_interval_secs: value.wallet_sync_interval_secs,
            fee_rate_cache_update_interval_secs: value.fee_rate_cache_update_interval_secs,
            trusted_peers_0conf: trusted_peers_0conf?,
            log_level: value.log_level.into(),
            probing_liquidity_limit_multiplier: value.probing_liquidity_limit_multiplier,
        })
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            storage_dir_path: DEFAULT_STORAGE_DIR_PATH.to_string(),
            log_dir_path: None,
            network: DEFAULT_NETWORK,
            listening_addresses: None,
            default_cltv_expiry_delta: DEFAULT_CLTV_EXPIRY_DELTA,
            onchain_wallet_sync_interval_secs: DEFAULT_BDK_WALLET_SYNC_INTERVAL_SECS,
            wallet_sync_interval_secs: DEFAULT_LDK_WALLET_SYNC_INTERVAL_SECS,
            fee_rate_cache_update_interval_secs: DEFAULT_FEE_RATE_CACHE_UPDATE_INTERVAL_SECS,
            trusted_peers_0conf: vec![],
            probing_liquidity_limit_multiplier: 3,
            log_level: DEFAULT_LOG_LEVEL,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChainDataSourceConfig {
    Esplora(String),
}

#[derive(Debug, Clone)]
pub enum EntropySourceConfig {
    SeedFile(String),
    SeedBytes([u8; 64]),
    Bip39Mnemonic {
        mnemonic: LdkMnemonic,
        passphrase: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum GossipSourceConfig {
    P2PNetwork,
    RapidGossipSync(String),
}

// Config defaults
const DEFAULT_STORAGE_DIR_PATH: &str = "/tmp/ldk_node/";
const DEFAULT_NETWORK: Network = Network::Testnet;
const DEFAULT_CLTV_EXPIRY_DELTA: u32 = 144;
const DEFAULT_BDK_WALLET_SYNC_INTERVAL_SECS: u64 = 60;
const DEFAULT_LDK_WALLET_SYNC_INTERVAL_SECS: u64 = 20;
const DEFAULT_FEE_RATE_CACHE_UPDATE_INTERVAL_SECS: u64 = 60;
const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Debug;
