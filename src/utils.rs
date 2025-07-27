use napi::bindgen_prelude::*;
use std::net::IpAddr;
use std::time::Duration;
use surge_ping::{Client, Config, IcmpPacket, PingSequence, Pinger};

use crate::types::PingResult;

/// Payload size for ping packets (56 bytes, standard ping size)
pub const PAYLOAD_SIZE: usize = 56;
/// Timeout duration for ping operations (5 seconds)
pub const PING_TIMEOUT_SECS: u64 = 5;

/// Checks if a string is a valid IP address (IPv4 or IPv6)
///
/// # Arguments
/// * `host` - The string to check
///
/// # Returns
/// Some(IpAddr) if the string is a valid IP address, None otherwise
pub fn is_ip_address(host: &str) -> Option<IpAddr> {
  host.parse::<IpAddr>().ok()
}

/// Resolves a hostname or IP address string to an IpAddr
///
/// # Arguments
/// * `host` - The hostname or IP address to resolve
///
/// # Returns
/// The resolved IP address or an error if resolution fails
///
/// # Behavior
/// - First attempts to parse as a direct IP address
/// - If that fails, performs DNS lookup using system resolver
/// - Returns the first available IP address from DNS results
pub async fn resolve_host(host: &str) -> Result<IpAddr> {
  // Fast path: if it's already an IP address, return it directly
  if let Some(ip) = is_ip_address(host) {
    return Ok(ip);
  }

  // Slow path: perform DNS lookup
  tokio::task::spawn_blocking({
    let host = host.to_string();
    move || {
      dns_lookup::lookup_host(&host)
        .map_err(|e| Error::new(Status::GenericFailure, format!("DNS lookup failed: {}", e)))
        .and_then(|all_ips| {
          if let Some(ip) = all_ips.first() {
            return Ok(*ip);
          }

          Err(Error::new(
            Status::GenericFailure,
            "No valid IP address found",
          ))
        })
    }
  })
  .await
  .map_err(|e| Error::new(Status::GenericFailure, format!("Task join error: {}", e)))?
}

/// Creates an ICMP client for the given IP address
///
/// # Arguments
/// * `ip` - The IP address to create a client for (IPv4 or IPv6)
///
/// # Returns
/// A configured ICMP client or an error if creation fails
///
/// # Notes
/// - Automatically detects IPv4 vs IPv6 and configures appropriate ICMP type
/// - May require elevated privileges on some systems
pub fn create_icmp_client(ip: IpAddr) -> Result<Client> {
  let icmp_kind = match ip {
    IpAddr::V4(_) => surge_ping::ICMP::V4,
    IpAddr::V6(_) => surge_ping::ICMP::V6,
  };

  let config = Config {
    kind: icmp_kind,
    ..Default::default()
  };

  Client::new(&config).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to create client: {}", e),
    )
  })
}

/// Creates a PingResult indicating a failed ping operation
///
/// # Arguments
/// * `host` - The original hostname that was pinged
/// * `ip` - The resolved IP address (may be empty if DNS failed)
/// * `seq` - The ICMP sequence number
/// * `error` - The error message describing what went wrong
///
/// # Returns
/// A PingResult with success=false and the provided error information
pub fn create_error_result(host: &str, ip: &str, seq: u32, error: String) -> PingResult {
  PingResult {
    host: host.to_string(),
    ip: ip.to_string(),
    bytes: 0,
    icmp_seq: seq,
    ttl: None,
    time: 0.0,
    success: false,
    error: Some(error),
  }
}

/// Creates a PingResult indicating a successful ping operation
///
/// # Arguments
/// * `host` - The original hostname that was pinged
/// * `pinger_host` - The actual IP address that responded
/// * `payload_len` - The size of the ping payload in bytes
/// * `_seq` - The sequence number (unused, kept for API compatibility)
/// * `elapsed` - The round-trip time duration
/// * `packet` - The received ICMP packet containing response data
///
/// # Returns
/// A PingResult with success=true and timing/packet information
pub fn create_success_result(
  host: &str,
  pinger_host: &str,
  payload_len: usize,
  _seq: u32,
  elapsed: Duration,
  packet: &IcmpPacket,
) -> PingResult {
  let (icmp_seq, ttl) = match packet {
    IcmpPacket::V4(packet) => (
      packet.get_sequence().0 as u32,
      packet.get_ttl().map(|t| t as u32),
    ),
    IcmpPacket::V6(packet) => (
      packet.get_sequence().0 as u32,
      Some(packet.get_max_hop_limit() as u32),
    ),
  };

  PingResult {
    host: host.to_string(),
    ip: pinger_host.to_string(),
    bytes: payload_len as u32,
    icmp_seq,
    ttl,
    time: elapsed.as_secs_f64() * 1000.0,
    success: true,
    error: None,
  }
}

/// Executes a single ping operation with timeout handling
///
/// # Arguments
/// * `pinger` - The pinger instance to use for sending the ping
/// * `host` - The original hostname being pinged
/// * `ip` - The resolved IP address
/// * `seq` - The ICMP sequence number for this ping
/// * `payload` - The payload data to send
///
/// # Returns
/// A PingResult containing either success data or error information
///
/// # Behavior
/// - Sends an ICMP ping packet with the specified payload
/// - Waits for response with a configurable timeout
/// - Returns appropriate success or error result
pub async fn execute_ping(
  pinger: &mut Pinger,
  host: &str,
  ip: IpAddr,
  seq: PingSequence,
  payload: &[u8],
) -> PingResult {
  match tokio::time::timeout(Duration::from_secs(PING_TIMEOUT_SECS), async {
    pinger.ping(seq, payload).await
  })
  .await
  {
    Ok(Ok((packet, duration))) => {
      let elapsed = duration;
      create_success_result(
        host,
        &pinger.host.to_string(),
        payload.len(),
        seq.0 as u32,
        elapsed,
        &packet,
      )
    }
    Ok(Err(e)) => create_error_result(host, &ip.to_string(), seq.0 as u32, e.to_string()),
    Err(_) => create_error_result(host, &ip.to_string(), seq.0 as u32, "Timeout".to_string()),
  }
}
