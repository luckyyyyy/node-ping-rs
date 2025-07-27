use napi::bindgen_prelude::*;
use napi_derive::napi;
use surge_ping::{PingIdentifier, PingSequence};

use crate::types::PingResult;
use crate::utils::{PAYLOAD_SIZE, create_icmp_client, execute_ping, resolve_host};

/// Performs a single ping operation to the specified host
///
/// # Arguments
/// * `host` - The hostname or IP address to ping
///
/// # Returns
/// A PingResult containing the ping statistics or error information
///
/// # Examples
/// ```javascript
/// const result = await ping('google.com');
/// console.log(result.success); // true or false
/// ```
#[napi]
pub async fn ping(host: String) -> Result<PingResult> {
  // // Optimize: Check if host is already an IP address to avoid unnecessary DNS lookup
  let ip = resolve_host(&host).await?;

  let client = create_icmp_client(ip)?;
  let mut pinger = client.pinger(ip, PingIdentifier(rand::random())).await;

  let payload = [0; PAYLOAD_SIZE];
  let seq = PingSequence(1);

  let result = execute_ping(&mut pinger, &host, ip, seq, &payload).await;
  Ok(result)
}
