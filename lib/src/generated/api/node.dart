// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.31.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'error.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'types.dart';

// Rust type: RustOpaqueNom<Node>
@sealed
class Node extends RustOpaque {
  Node.dcoDecode(List<dynamic> wire) : super.dcoDecode(wire, _kStaticData);

  Node.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        LdkCore.instance.api.rust_arc_increment_strong_count_Node,
    rustArcDecrementStrongCount:
        LdkCore.instance.api.rust_arc_decrement_strong_count_Node,
    rustArcDecrementStrongCountPtr:
        LdkCore.instance.api.rust_arc_decrement_strong_count_NodePtr,
  );
}

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<NodeBuilder>>
@sealed
class NodeBuilder extends RustOpaque {
  NodeBuilder.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  NodeBuilder.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        LdkCore.instance.api.rust_arc_increment_strong_count_NodeBuilder,
    rustArcDecrementStrongCount:
        LdkCore.instance.api.rust_arc_decrement_strong_count_NodeBuilder,
    rustArcDecrementStrongCountPtr:
        LdkCore.instance.api.rust_arc_decrement_strong_count_NodeBuilderPtr,
  );

  Future<LdkNode> build({dynamic hint}) =>
      LdkCore.instance.api.nodeBuilderBuild(that: this, hint: hint);

  Future<LdkNode> buildWithFsStore({dynamic hint}) =>
      LdkCore.instance.api.nodeBuilderBuildWithFsStore(that: this, hint: hint);

  static Future<NodeBuilder> createBuilder(
          {required Config config,
          ChainDataSourceConfig? chainDataSourceConfig,
          EntropySourceConfig? entropySourceConfig,
          GossipSourceConfig? gossipSourceConfig,
          LiquiditySourceConfig? liquiditySourceConfig,
          dynamic hint}) =>
      LdkCore.instance.api.nodeBuilderCreateBuilder(
          config: config,
          chainDataSourceConfig: chainDataSourceConfig,
          entropySourceConfig: entropySourceConfig,
          gossipSourceConfig: gossipSourceConfig,
          liquiditySourceConfig: liquiditySourceConfig,
          hint: hint);
}

// Rust type: RustOpaqueNom<ldk_node :: payment :: Bolt11Payment>
@sealed
class LdkNodePaymentBolt11Payment extends RustOpaque {
  LdkNodePaymentBolt11Payment.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  LdkNodePaymentBolt11Payment.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: LdkCore.instance.api
        .rust_arc_increment_strong_count_LdkNodePaymentBolt11Payment,
    rustArcDecrementStrongCount: LdkCore.instance.api
        .rust_arc_decrement_strong_count_LdkNodePaymentBolt11Payment,
    rustArcDecrementStrongCountPtr: LdkCore.instance.api
        .rust_arc_decrement_strong_count_LdkNodePaymentBolt11PaymentPtr,
  );
}

// Rust type: RustOpaqueNom<ldk_node :: payment :: OnchainPayment>
@sealed
class LdkNodePaymentOnchainPayment extends RustOpaque {
  LdkNodePaymentOnchainPayment.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  LdkNodePaymentOnchainPayment.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: LdkCore.instance.api
        .rust_arc_increment_strong_count_LdkNodePaymentOnchainPayment,
    rustArcDecrementStrongCount: LdkCore.instance.api
        .rust_arc_decrement_strong_count_LdkNodePaymentOnchainPayment,
    rustArcDecrementStrongCountPtr: LdkCore.instance.api
        .rust_arc_decrement_strong_count_LdkNodePaymentOnchainPaymentPtr,
  );
}

// Rust type: RustOpaqueNom<ldk_node :: payment :: SpontaneousPayment>
@sealed
class LdkNodePaymentSpontaneousPayment extends RustOpaque {
  LdkNodePaymentSpontaneousPayment.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  LdkNodePaymentSpontaneousPayment.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: LdkCore.instance.api
        .rust_arc_increment_strong_count_LdkNodePaymentSpontaneousPayment,
    rustArcDecrementStrongCount: LdkCore.instance.api
        .rust_arc_decrement_strong_count_LdkNodePaymentSpontaneousPayment,
    rustArcDecrementStrongCountPtr: LdkCore.instance.api
        .rust_arc_decrement_strong_count_LdkNodePaymentSpontaneousPaymentPtr,
  );
}

class LdkBolt11Payment {
  final LdkNodePaymentBolt11Payment ptr;

  const LdkBolt11Payment({
    required this.ptr,
  });

  Future<Bolt11Invoice> receive(
          {required int amountMsat,
          required String description,
          required int expirySecs,
          dynamic hint}) =>
      LdkCore.instance.api.ldkBolt11PaymentReceive(
          that: this,
          amountMsat: amountMsat,
          description: description,
          expirySecs: expirySecs,
          hint: hint);

  Future<Bolt11Invoice> receiveVariableAmount(
          {required String description,
          required int expirySecs,
          dynamic hint}) =>
      LdkCore.instance.api.ldkBolt11PaymentReceiveVariableAmount(
          that: this,
          description: description,
          expirySecs: expirySecs,
          hint: hint);

  Future<Bolt11Invoice> receiveVariableAmountViaJitChannel(
          {required String description,
          required int expirySecs,
          int? maxProportionalLspFeeLimitPpmMsat,
          dynamic hint}) =>
      LdkCore.instance.api.ldkBolt11PaymentReceiveVariableAmountViaJitChannel(
          that: this,
          description: description,
          expirySecs: expirySecs,
          maxProportionalLspFeeLimitPpmMsat: maxProportionalLspFeeLimitPpmMsat,
          hint: hint);

  Future<Bolt11Invoice> receiveViaJitChannel(
          {required int amountMsat,
          required String description,
          required int expirySecs,
          int? maxTotalLspFeeLimitMsat,
          dynamic hint}) =>
      LdkCore.instance.api.ldkBolt11PaymentReceiveViaJitChannel(
          that: this,
          amountMsat: amountMsat,
          description: description,
          expirySecs: expirySecs,
          maxTotalLspFeeLimitMsat: maxTotalLspFeeLimitMsat,
          hint: hint);

  Future<PaymentId> send({required Bolt11Invoice invoice, dynamic hint}) =>
      LdkCore.instance.api
          .ldkBolt11PaymentSend(that: this, invoice: invoice, hint: hint);

  Future<void> sendProbes({required Bolt11Invoice invoice, dynamic hint}) =>
      LdkCore.instance.api
          .ldkBolt11PaymentSendProbes(that: this, invoice: invoice, hint: hint);

  Future<void> sendProbesUsingAmount(
          {required Bolt11Invoice invoice,
          required int amountMsat,
          dynamic hint}) =>
      LdkCore.instance.api.ldkBolt11PaymentSendProbesUsingAmount(
          that: this, invoice: invoice, amountMsat: amountMsat, hint: hint);

  Future<PaymentId> sendUsingAmount(
          {required Bolt11Invoice invoice,
          required int amountMsat,
          dynamic hint}) =>
      LdkCore.instance.api.ldkBolt11PaymentSendUsingAmount(
          that: this, invoice: invoice, amountMsat: amountMsat, hint: hint);

  @override
  int get hashCode => ptr.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LdkBolt11Payment &&
          runtimeType == other.runtimeType &&
          ptr == other.ptr;
}

class LdkMnemonic {
  final String seedPhrase;

  const LdkMnemonic({
    required this.seedPhrase,
  });

  static Future<LdkMnemonic> generate({dynamic hint}) =>
      LdkCore.instance.api.ldkMnemonicGenerate(hint: hint);

  @override
  int get hashCode => seedPhrase.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LdkMnemonic &&
          runtimeType == other.runtimeType &&
          seedPhrase == other.seedPhrase;
}

class LdkNode {
  final Node ptr;

  const LdkNode({
    required this.ptr,
  });

  static Future<LdkBolt11Payment> bolt11Payment(
          {required LdkNode ptr, dynamic hint}) =>
      LdkCore.instance.api.ldkNodeBolt11Payment(ptr: ptr, hint: hint);

  Future<void> closeChannel(
          {required UserChannelId userChannelId,
          required PublicKey counterpartyNodeId,
          dynamic hint}) =>
      LdkCore.instance.api.ldkNodeCloseChannel(
          that: this,
          userChannelId: userChannelId,
          counterpartyNodeId: counterpartyNodeId,
          hint: hint);

  Future<Config> config({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeConfig(that: this, hint: hint);

  Future<void> connect(
          {required PublicKey nodeId,
          required SocketAddress address,
          required bool persist,
          dynamic hint}) =>
      LdkCore.instance.api.ldkNodeConnect(
          that: this,
          nodeId: nodeId,
          address: address,
          persist: persist,
          hint: hint);

  Future<UserChannelId> connectOpenChannel(
          {required SocketAddress socketAddress,
          required PublicKey nodeId,
          required int channelAmountSats,
          int? pushToCounterpartyMsat,
          required bool announceChannel,
          ChannelConfig? channelConfig,
          dynamic hint}) =>
      LdkCore.instance.api.ldkNodeConnectOpenChannel(
          that: this,
          socketAddress: socketAddress,
          nodeId: nodeId,
          channelAmountSats: channelAmountSats,
          pushToCounterpartyMsat: pushToCounterpartyMsat,
          announceChannel: announceChannel,
          channelConfig: channelConfig,
          hint: hint);

  Future<void> disconnect(
          {required PublicKey counterpartyNodeId, dynamic hint}) =>
      LdkCore.instance.api.ldkNodeDisconnect(
          that: this, counterpartyNodeId: counterpartyNodeId, hint: hint);

  Future<void> eventHandled({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeEventHandled(that: this, hint: hint);

  Future<BalanceDetails> listBalances({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeListBalances(that: this, hint: hint);

  Future<List<ChannelDetails>> listChannels({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeListChannels(that: this, hint: hint);

  Future<List<PaymentDetails>> listPayments({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeListPayments(that: this, hint: hint);

  Future<List<PaymentDetails>> listPaymentsWithFilter(
          {required PaymentDirection paymentDirection, dynamic hint}) =>
      LdkCore.instance.api.ldkNodeListPaymentsWithFilter(
          that: this, paymentDirection: paymentDirection, hint: hint);

  Future<List<PeerDetails>> listPeers({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeListPeers(that: this, hint: hint);

  Future<List<SocketAddress>?> listeningAddresses({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeListeningAddresses(that: this, hint: hint);

  Future<Event?> nextEvent({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeNextEvent(that: this, hint: hint);

  Future<Event> nextEventAsync({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeNextEventAsync(that: this, hint: hint);

  Future<PublicKey> nodeId({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeNodeId(that: this, hint: hint);

  static Future<LdkOnChainPayment> onChainPayment(
          {required LdkNode ptr, dynamic hint}) =>
      LdkCore.instance.api.ldkNodeOnChainPayment(ptr: ptr, hint: hint);

  Future<PaymentDetails?> payment(
          {required PaymentId paymentId, dynamic hint}) =>
      LdkCore.instance.api
          .ldkNodePayment(that: this, paymentId: paymentId, hint: hint);

  Future<void> removePayment({required PaymentId paymentId, dynamic hint}) =>
      LdkCore.instance.api
          .ldkNodeRemovePayment(that: this, paymentId: paymentId, hint: hint);

  Future<String> signMessage({required List<int> msg, dynamic hint}) =>
      LdkCore.instance.api.ldkNodeSignMessage(that: this, msg: msg, hint: hint);

  static Future<LdkSpontaneousPayment> spontaneousPayment(
          {required LdkNode ptr, dynamic hint}) =>
      LdkCore.instance.api.ldkNodeSpontaneousPayment(ptr: ptr, hint: hint);

  Future<void> start({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeStart(that: this, hint: hint);

  Future<NodeStatus> status({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeStatus(that: this, hint: hint);

  Future<void> stop({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeStop(that: this, hint: hint);

  Future<void> syncWallets({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeSyncWallets(that: this, hint: hint);

  Future<void> updateChannelConfig(
          {required UserChannelId userChannelId,
          required PublicKey counterpartyNodeId,
          required ChannelConfig channelConfig,
          dynamic hint}) =>
      LdkCore.instance.api.ldkNodeUpdateChannelConfig(
          that: this,
          userChannelId: userChannelId,
          counterpartyNodeId: counterpartyNodeId,
          channelConfig: channelConfig,
          hint: hint);

  Future<bool> verifySignature(
          {required List<int> msg,
          required String sig,
          required PublicKey publicKey,
          dynamic hint}) =>
      LdkCore.instance.api.ldkNodeVerifySignature(
          that: this, msg: msg, sig: sig, publicKey: publicKey, hint: hint);

  Future<Event> waitNextEvent({dynamic hint}) =>
      LdkCore.instance.api.ldkNodeWaitNextEvent(that: this, hint: hint);

  @override
  int get hashCode => ptr.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LdkNode && runtimeType == other.runtimeType && ptr == other.ptr;
}

class LdkOnChainPayment {
  final LdkNodePaymentOnchainPayment ptr;

  const LdkOnChainPayment({
    required this.ptr,
  });

  Future<Address> newAddress({dynamic hint}) =>
      LdkCore.instance.api.ldkOnChainPaymentNewAddress(that: this, hint: hint);

  Future<Txid> sendAllToAddress({required Address address, dynamic hint}) =>
      LdkCore.instance.api.ldkOnChainPaymentSendAllToAddress(
          that: this, address: address, hint: hint);

  Future<Txid> sendToAddress(
          {required Address address, required int amountSats, dynamic hint}) =>
      LdkCore.instance.api.ldkOnChainPaymentSendToAddress(
          that: this, address: address, amountSats: amountSats, hint: hint);

  @override
  int get hashCode => ptr.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LdkOnChainPayment &&
          runtimeType == other.runtimeType &&
          ptr == other.ptr;
}

class LdkSpontaneousPayment {
  final LdkNodePaymentSpontaneousPayment ptr;

  const LdkSpontaneousPayment({
    required this.ptr,
  });

  @override
  int get hashCode => ptr.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LdkSpontaneousPayment &&
          runtimeType == other.runtimeType &&
          ptr == other.ptr;
}

class U8Array12 extends NonGrowableListView<int> {
  static const arraySize = 12;

  @internal
  Uint8List get inner => _inner;
  final Uint8List _inner;

  U8Array12(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  U8Array12.init() : this(Uint8List(arraySize));
}

class U8Array16 extends NonGrowableListView<int> {
  static const arraySize = 16;

  @internal
  Uint8List get inner => _inner;
  final Uint8List _inner;

  U8Array16(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  U8Array16.init() : this(Uint8List(arraySize));
}

class U8Array32 extends NonGrowableListView<int> {
  static const arraySize = 32;

  @internal
  Uint8List get inner => _inner;
  final Uint8List _inner;

  U8Array32(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  U8Array32.init() : this(Uint8List(arraySize));
}

class U8Array4 extends NonGrowableListView<int> {
  static const arraySize = 4;

  @internal
  Uint8List get inner => _inner;
  final Uint8List _inner;

  U8Array4(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  U8Array4.init() : this(Uint8List(arraySize));
}

class U8Array64 extends NonGrowableListView<int> {
  static const arraySize = 64;

  @internal
  Uint8List get inner => _inner;
  final Uint8List _inner;

  U8Array64(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  U8Array64.init() : this(Uint8List(arraySize));
}