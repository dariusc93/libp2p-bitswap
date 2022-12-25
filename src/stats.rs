use once_cell::sync::Lazy;
use prometheus::{HistogramOpts, HistogramVec, IntCounter, IntCounterVec, Opts};

pub static REQUESTS_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(
        Opts::new(
            "bitswap_requests_total",
            "Number of bitswap requests labelled by type and result.",
        ),
        &["type"],
    )
    .unwrap()
});

pub static REQUEST_DURATION_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    HistogramVec::new(
        HistogramOpts::new(
            "bitswap_request_duration_seconds",
            "Duration of bitswap requests labelled by request type",
        ),
        &["type"],
    )
    .unwrap()
});

pub static REQUESTS_CANCELED: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new(
        "bitswap_requests_canceled_total",
        "Number of canceled requests",
    )
    .unwrap()
});

pub static BLOCK_NOT_FOUND: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new(
        "bitswap_block_not_found_total",
        "Number of block not found errors.",
    )
    .unwrap()
});

pub static PROVIDERS_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new(
        "bitswap_providers_total",
        r#"Number of providers total. Using the number of provider requests, the average
        number of providers per request can be computed."#,
    )
    .unwrap()
});

pub static MISSING_BLOCKS_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new(
        "bitswap_missing_blocks_total",
        r#"Number of missing blocks total. Using the number of missing blocks requests, the
        average number of missing blocks per request can be computed."#,
    )
    .unwrap()
});

pub static RECEIVED_BLOCK_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("bitswap_received_block_bytes", "Number of received bytes.").unwrap()
});

pub static RECEIVED_INVALID_BLOCK_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new(
        "bitswap_received_invalid_block_bytes",
        "Number of received bytes that didn't match the hash.",
    )
    .unwrap()
});

pub static SENT_BLOCK_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("bitswap_sent_block_bytes", "Number of sent block bytes.").unwrap()
});

pub static RESPONSES_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(
        Opts::new(
            "bitswap_responses_total",
            "Number of bitswap responses sent to peers.",
        ),
        &["type"],
    )
    .unwrap()
});

pub static THROTTLED_INBOUND: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new(
        "bitswap_throttled_too_many_inbound_total",
        "Number of too many inbound events.",
    )
    .unwrap()
});

pub static THROTTLED_OUTBOUND: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new(
        "bitswap_throttled_resume_send_total",
        "Number of resume send events.",
    )
    .unwrap()
});

pub static OUTBOUND_FAILURE: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(
        Opts::new(
            "bitswap_outbound_failures_total",
            "Number of outbound failures.",
        ),
        &["type"],
    )
    .unwrap()
});

pub static INBOUND_FAILURE: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(
        Opts::new(
            "bitswap_inbound_failures_total",
            "Number of inbound failures.",
        ),
        &["type"],
    )
    .unwrap()
});
