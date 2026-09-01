#![no_main]

use alloy_sol_types::{SolCall, SolType, abi, abi::AbiDecoderConfig, sol_data};
use libfuzzer_sys::fuzz_target;

// Dynamic ABI signatures copied from the Zones and Tempo precompile bindings.
// Static-only calls are covered by the primitive members of these signatures;
// `fixedBytes` fills the fixed-array gap in those bindings.
alloy_sol_types::sol! {
    struct QueuedDeposit { uint8 depositType; bytes depositData; bool rejected; }
    struct ChaumPedersenProof { bytes32 s; bytes32 c; }
    struct DecryptionData {
        bytes32 sharedSecret;
        uint8 sharedSecretYParity;
        ChaumPedersenProof cpProof;
    }
    struct EnabledToken { address token; string name; string symbol; string currency; }

    // zones/crates/contracts/src/precompiles/zone_inbox.rs
    function advanceTempo(bytes header, QueuedDeposit[] deposits, DecryptionData[] decryptions, EnabledToken[] enabledTokens);

    struct ZoneInfo {
        uint32 zoneId;
        address portal;
        bool accessMode;
        bool gatewayMode;
        address admin;
        address[] sequencers;
        uint8 threshold;
        address verifier;
        string rpcUrl;
    }
    struct CreateZoneParams {
        address initialToken;
        bool accessMode;
        bool gatewayMode;
        address[] allowedAccounts;
        address[] zoneGateways;
        address admin;
        address[] sequencers;
        uint8 threshold;
        string rpcUrl;
    }

    // zones/crates/contracts/src/precompiles/zone_factory.rs
    function createZone(CreateZoneParams params) returns (uint32 zoneId, address portal);
    function zones(uint32 zoneId) returns (ZoneInfo info);

    struct PendingWithdrawal {
        address token;
        address sender;
        bytes32 txHash;
        address to;
        uint128 amount;
        bytes32 memo;
        uint64 gasLimit;
        uint64 fallbackNonce;
        bytes callbackData;
        bytes revealTo;
    }

    // zones/crates/contracts/src/precompiles/outbox.rs
    function requestWithdrawal(address token, address to, uint128 amount, bytes32 memo, uint64 gasLimit, address zoneFallbackRecipient, bytes data, bytes revealTo);
    function finalizeWithdrawalBatch(uint256 count, uint64 blockNumber, bytes[] encryptedSenders) returns (bytes32 withdrawalQueueHash);
    function getPendingWithdrawals() returns (PendingWithdrawal[] withdrawals);

    struct DepositPayload { bytes32 ephemeralPubkeyX; uint8 ephemeralPubkeyYParity; bytes ciphertext; bytes12 nonce; bytes16 tag; }
    struct Deposit { address token; address sender; uint128 amount; address tempoRefundRecipient; uint256 keyIndex; DepositPayload encrypted; }
    struct Withdrawal {
        address token;
        bytes32 senderTag;
        address to;
        uint128 amount;
        bytes32 memo;
        uint64 gasLimit;
        uint64 fallbackNonce;
        bytes callbackData;
        bytes encryptedSender;
    }
    struct BlockTransition { bytes32 prevBlockHash; bytes32 nextBlockHash; }
    struct DepositQueueTransition { bytes32 prevProcessedHash; bytes32 nextProcessedHash; uint64 prevDepositNumber; uint64 nextDepositNumber; }

    // zones/crates/contracts/src/precompiles/zone_portal.rs
    function submitBatch(uint64 tempoBlockNumber, uint64 recentTempoBlockNumber, BlockTransition blockTransition, DepositQueueTransition depositTransition, bytes32 withdrawalQueueHash, bytes verifierConfig, bytes proof, uint256 nextZoneHeight, bytes[] signatures);

    struct Validator { address validatorAddress; bytes32 publicKey; bool active; string inboundAddress; string outboundAddress; }

    // tempo/crates/contracts/src/precompiles/validator_config.rs
    function addValidator(address newValidatorAddress, bytes32 publicKey, bool active, string inboundAddress, string outboundAddress);
    function getValidators() returns (Validator[] validators);

    struct ClaimReceiptV1 {
        uint8 version;
        address token;
        address recoveryAuthority;
        address originator;
        address recipient;
        uint64 blockedAt;
        uint64 blockedNonce;
        uint8 blockedReason;
        uint8 kind;
        bytes32 memo;
    }

    // tempo/crates/contracts/src/precompiles/receive_policy_guard.rs
    function claim(address to, bytes receipt);
    function balanceOf(bytes receipt) returns (uint256 amount);

    struct ChannelDescriptor { address payer; address payee; address operator; address token; bytes32 salt; address authorizedSigner; bytes32 expiringNonceHash; }
    struct ChannelState { uint96 settled; uint96 deposit; uint32 closeRequestedAt; }

    // tempo/crates/contracts/src/precompiles/tip20_channel_reserve.rs
    function open(address payee, address operator, address token, uint96 deposit, bytes32 salt, address authorizedSigner) returns (bytes32 channelId);
    function settle(ChannelDescriptor descriptor, uint96 cumulativeAmount, bytes signature);
    function getChannelStatesBatch(bytes32[] channelIds) returns (ChannelState[] states);

    struct CallScope { address target; SelectorRule[] selectorRules; }
    struct SelectorRule { bytes4 selector; address[] recipients; }
    struct TokenLimit { address token; uint256 amount; uint64 period; }
    struct KeyRestrictions { uint64 expiry; bool enforceLimits; TokenLimit[] limits; bool allowAnyCalls; CallScope[] allowedCalls; }

    // tempo/crates/contracts/src/precompiles/account_keychain.rs
    function authorizeKey(address keyId, uint8 signatureType, KeyRestrictions config);
    function setAllowedCalls(address keyId, CallScope[] scopes);

    // tempo/crates/contracts/src/precompiles/tip20_factory.rs
    function createToken(string name, string symbol, string currency, uint8 decimals, address quoteToken, uint256 supplyCap) returns (address token);

    // Coverage for a fixed array of dynamic values, absent from the copied ABI.
    function fixedBytes(bytes[2] values);
}

type ZonesTempoShape = (
    sol_data::Bytes,
    sol_data::String,
    sol_data::Array<sol_data::Bytes>,
    sol_data::Array<sol_data::Array<sol_data::Bytes>>,
    sol_data::FixedArray<sol_data::Bytes, 2>,
    (sol_data::Address, sol_data::Uint<128>, sol_data::Bytes),
);
type EncoderShape = (
    sol_data::Bytes,
    sol_data::Array<sol_data::Bytes>,
    sol_data::Array<sol_data::Array<sol_data::Bytes>>,
    sol_data::FixedArray<sol_data::Bytes, 2>,
);
type PrimitiveShape = (
    sol_data::Bool,
    sol_data::Int<16>,
    sol_data::Int<256>,
    sol_data::Uint<8>,
    sol_data::Uint<256>,
    sol_data::FixedBytes<4>,
    sol_data::FixedBytes<32>,
    sol_data::Address,
);
type StaticArrayShape = (
    sol_data::FixedArray<sol_data::Uint<256>, 2>,
    sol_data::FixedArray<sol_data::FixedArray<sol_data::Address, 2>, 2>,
);
type MixedShape = (
    sol_data::Uint<256>,
    sol_data::Bytes,
    sol_data::Address,
    sol_data::Array<sol_data::Bytes>,
    sol_data::Bool,
    sol_data::FixedArray<sol_data::Bytes, 2>,
);

fn assert_strict_roundtrip<T: SolType>(bytes: &[u8]) {
    let Ok(token) =
        abi::decode_with_config::<T::Token<'_>>(bytes, AbiDecoderConfig::new().strict(true))
    else {
        return;
    };
    assert_eq!(abi::encode(&token), bytes, "{}", T::SOL_NAME);
}

fn assert_strict_call<T: SolCall>(bytes: &[u8])
where
    for<'a> T::Token<'a>: alloy_sol_types::abi::TokenSeq<'a>,
{
    let Ok(token) = abi::decode_sequence_with_config::<T::Token<'_>>(
        bytes,
        AbiDecoderConfig::new().strict(true),
    ) else {
        return;
    };
    assert_eq!(abi::encode_sequence(&token), bytes, "{}", T::SIGNATURE);
}

fuzz_target!(|bytes: &[u8]| {
    assert_strict_roundtrip::<ZonesTempoShape>(bytes);
    assert_strict_roundtrip::<PrimitiveShape>(bytes);
    assert_strict_roundtrip::<StaticArrayShape>(bytes);
    assert_strict_roundtrip::<EncoderShape>(bytes);
    assert_strict_roundtrip::<MixedShape>(bytes);
    assert_strict_roundtrip::<QueuedDeposit>(bytes);
    assert_strict_roundtrip::<DecryptionData>(bytes);
    assert_strict_roundtrip::<EnabledToken>(bytes);
    assert_strict_roundtrip::<ZoneInfo>(bytes);
    assert_strict_roundtrip::<CreateZoneParams>(bytes);
    assert_strict_roundtrip::<PendingWithdrawal>(bytes);
    assert_strict_roundtrip::<Deposit>(bytes);
    assert_strict_roundtrip::<Withdrawal>(bytes);
    assert_strict_roundtrip::<Validator>(bytes);
    assert_strict_roundtrip::<ClaimReceiptV1>(bytes);
    assert_strict_roundtrip::<ChannelDescriptor>(bytes);
    assert_strict_roundtrip::<ChannelState>(bytes);
    assert_strict_roundtrip::<KeyRestrictions>(bytes);

    assert_strict_call::<advanceTempoCall>(bytes);
    assert_strict_call::<createZoneCall>(bytes);
    assert_strict_call::<requestWithdrawalCall>(bytes);
    assert_strict_call::<finalizeWithdrawalBatchCall>(bytes);
    assert_strict_call::<submitBatchCall>(bytes);
    assert_strict_call::<addValidatorCall>(bytes);
    assert_strict_call::<claimCall>(bytes);
    assert_strict_call::<openCall>(bytes);
    assert_strict_call::<settleCall>(bytes);
    assert_strict_call::<authorizeKeyCall>(bytes);
    assert_strict_call::<createTokenCall>(bytes);
    assert_strict_call::<fixedBytesCall>(bytes);
});
