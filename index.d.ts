/* auto-generated by NAPI-RS */
/* eslint-disable */
/**
 * Performs a single ping operation to the specified host
 *
 * # Arguments
 * * `host` - The hostname or IP address to ping
 *
 * # Returns
 * A PingResult containing the ping statistics or error information
 *
 * # Examples
 * ```javascript
 * const result = await ping('google.com');
 * console.log(result.success); // true or false
 * ```
 */
export declare function ping(host: string): Promise<PingResult>

/**
 * Result structure containing ping operation statistics and status
 *
 * This structure is returned by both single ping operations callbacks.
 * It contains comprehensive information about the ping attempt including timing data,
 * network path information, and success/failure status.
 */
export interface PingResult {
  /** The original hostname or IP address that was pinged */
  host: string
  /** The actual IP address that responded to the ping */
  ip: string
  /** The size of the ping payload in bytes */
  bytes: number
  /** The ICMP sequence number for this ping */
  icmpSeq: number
  /** Time-to-live (TTL) value from the response packet, if available */
  ttl?: number
  /** Round-trip time in milliseconds */
  time: number
  /** Whether the ping was successful */
  success: boolean
  /** Error message if the ping failed, None if successful */
  error?: string
}
