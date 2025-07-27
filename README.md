# node-ping-rs

[![CI](https://github.com/luckyyyyy/node-ping-rs/workflows/CI/badge.svg)](https://github.com/luckyyyyy/node-ping-rs/actions)
[![npm version](https://badge.fury.io/js/node-ping-rs.svg)](https://badge.fury.io/js/node-ping-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> A high-performance ping utility for Node.js built with Rust and napi-rs, supporting both single ping operations.

**⚠️ Important**: This library requires special permissions on Linux/macOS. See [Permission Requirements](#️-important-permission-requirements) section.

## Features

- 🚀 **High Performance**: Built with Rust for maximum performance
- 🌐 **IPv4 and IPv6 Support**: Works with both IPv4 and IPv6 addresses
- ⚡ **Async/Await**: Full async/await support
- 🛡️ **Type Safe**: Written in TypeScript with full type definitions
- 🏗️ **Cross Platform**: Supports Windows, macOS, and Linux

## Quick Start

1. Install the package:
```bash
npm install node-ping-rs
```

2. On Linux/macOS, grant permissions:
```bash
sudo setcap cap_net_raw+ep $(which node)
```

3. Use in your code:
```javascript
import { ping } from 'node-ping-rs';
const result = await ping('google.com');
console.log(result.success); // true
```

## Usage Examples

### Single Ping

```javascript
import { ping } from 'node-ping-rs';

async function example() {
  try {
    const result = await ping('google.com');
    console.log(result);
    // {
    //   host: 'google.com',
    //   ip: '142.250.191.14',
    //   bytes: 56,
    //   icmp_seq: 1,
    //   ttl: 116,
    //   time: 12.5,
    //   success: true,
    //   error: null
    // }
  } catch (error) {
    console.error('Ping failed:', error);
  }
}
```


## ⚠️ Important: Permission Requirements

**This library requires special permissions to send ICMP packets on most systems.**

### Linux/macOS

On Unix-like systems, ICMP sockets require elevated privileges. Choose one of the following approaches:

#### Option 1: Set capabilities on Node.js binary (Recommended for Development)
```bash
# Grant raw socket capabilities to Node.js
sudo setcap cap_net_raw+ep $(which node)

# Verify the capability was set
getcap $(which node)
# Should output: /path/to/node = cap_net_raw+ep
```

**💡 Quick Check**: Use our permission checker script:
```bash
# Download and run the permission checker
curl -s https://raw.githubusercontent.com/luckyyyyy/node-ping-rs/main/scripts/check-permissions.sh | bash
# Or if you've cloned the repo:
./scripts/check-permissions.sh
```

#### Option 2: Run with sudo (Not Recommended for Production)
```bash
sudo node your-app.js
```

#### Option 3: Enable unprivileged ICMP sockets (System-wide)
```bash
# Temporarily (until reboot)
sudo sysctl -w net.ipv4.ping_group_range="0 2147483647"

# Permanently
echo 'net.ipv4.ping_group_range = 0 2147483647' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

#### Option 4: Use Docker with proper capabilities
```dockerfile
FROM node:18
# Add NET_RAW capability
# docker run --cap-add=NET_RAW your-image
```

### Windows

On Windows, no special permissions are required as the library uses the system's ping functionality.

### Production Deployment Notes

- **Docker**: Use `--cap-add=NET_RAW` capability instead of running as root
- **PM2/Forever**: Set capabilities on Node.js binary before starting the process manager
- **systemd**: Configure the service with `AmbientCapabilities=CAP_NET_RAW`

## API Reference

### `ping(host: string): Promise<PingResult>`

Performs a single ping operation.

**Parameters:**
- `host` - The hostname or IP address to ping

**Returns:** Promise that resolves to a `PingResult` object

### `PingResult`

```typescript
interface PingResult {
  host: string;        // Original hostname
  ip: string;          // Resolved IP address
  bytes: number;       // Payload size in bytes
  icmp_seq: number;    // ICMP sequence number
  ttl?: number;        // Time to live
  time: number;        // Round-trip time in milliseconds
  success: boolean;    // Whether the ping was successful
  error?: string;      // Error message if unsuccessful
}
```

## Error Handling

The library handles various error conditions:

- **DNS Resolution Failures**: When hostname cannot be resolved
- **Network Unreachable**: When target host is unreachable
- **Permission Denied**: When insufficient privileges for ICMP
- **Timeout**: When ping request times out (5 seconds default)

## Development

### Prerequisites

- **Rust**: Install the latest stable Rust toolchain
- **Node.js**: Version 12.22.0+ with full Node-API support
- **Build Tools**: Platform-specific build tools (see below)

### Platform-specific Build Requirements

#### Linux
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# CentOS/RHEL/Fedora
sudo yum groupinstall "Development Tools"
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### Windows
- Visual Studio 2019 or later with C++ build tools
- Or Visual Studio Build Tools 2019

### Building from Source

```bash
# Clone the repository
git clone https://github.com/luckyyyyy/node-ping-rs.git
cd node-ping-rs

# Install dependencies
yarn install

# Build the native module
yarn build

# Run tests
yarn test

# Run benchmarks
yarn bench
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Built with [napi-rs](https://napi.rs/) for Node.js native addon development
- Uses [surge-ping](https://crates.io/crates/surge-ping) for ICMP ping functionality
- Inspired by the need for high-performance network utilities in Node.js

## Test in local

- yarn
- yarn build
- yarn test

And you will see:

```bash
$ ava --verbose

  ✔ sync function from native code
  ✔ sleep function from native code (201ms)
  ─

  2 tests passed
✨  Done in 1.12s.
```

## Release package

Ensure you have set your **NPM_TOKEN** in the `GitHub` project setting.

In `Settings -> Secrets`, add **NPM_TOKEN** into it.

When you want to release the package:

```
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.
