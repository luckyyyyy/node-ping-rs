use napi_derive::napi;

/// Result structure containing ping operation statistics and status
///
/// This structure is returned by both single ping operations callbacks.
/// It contains comprehensive information about the ping attempt including timing data,
/// network path information, and success/failure status.
#[napi(object)]
pub struct PingResult {
  /// The original hostname or IP address that was pinged
  pub host: String,
  /// The actual IP address that responded to the ping
  pub ip: String,
  /// The size of the ping payload in bytes
  pub bytes: u32,
  /// The ICMP sequence number for this ping
  pub icmp_seq: u32,
  /// Time-to-live (TTL) value from the response packet, if available
  pub ttl: Option<u32>,
  /// Round-trip time in milliseconds
  pub time: f64,
  /// Whether the ping was successful
  pub success: bool,
  /// Error message if the ping failed, None if successful
  pub error: Option<String>,
}
