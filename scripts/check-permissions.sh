#!/bin/bash

# Permission Check Script for node-ping-rs
# This script helps verify and set up the required permissions for ICMP ping operations

echo "🔍 Checking permissions for node-ping-rs..."
echo "============================================="

# Check if running on supported OS
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    echo "✅ Windows detected - no special permissions required!"
    exit 0
fi

# Find Node.js executable
NODE_PATH=$(which node)
if [ -z "$NODE_PATH" ]; then
    echo "❌ Node.js not found in PATH"
    exit 1
fi

echo "📍 Node.js found at: $NODE_PATH"

# Check current capabilities
echo ""
echo "🔍 Checking current capabilities..."
CURRENT_CAPS=$(getcap "$NODE_PATH" 2>/dev/null)

if [[ "$CURRENT_CAPS" == *"cap_net_raw+ep"* ]]; then
    echo "✅ Node.js already has required capabilities!"
    echo "   Current capabilities: $CURRENT_CAPS"
    echo ""
    echo "🎉 You're ready to use node-ping-rs!"
    exit 0
fi

# Check unprivileged ping settings
echo ""
echo "🔍 Checking unprivileged ping settings..."
PING_GROUP_RANGE=$(sysctl -n net.ipv4.ping_group_range 2>/dev/null || echo "1 0")
MIN_GID=$(echo $PING_GROUP_RANGE | cut -d' ' -f1)
MAX_GID=$(echo $PING_GROUP_RANGE | cut -d' ' -f2)
CURRENT_GID=$(id -g)

if [ "$MIN_GID" -le "$CURRENT_GID" ] && [ "$CURRENT_GID" -le "$MAX_GID" ]; then
    echo "✅ Unprivileged ping is enabled for your user group!"
    echo "   Group range: $PING_GROUP_RANGE"
    echo "   Your GID: $CURRENT_GID"
    echo ""
    echo "🎉 You're ready to use node-ping-rs!"
    exit 0
fi

# Provide setup instructions
echo ""
echo "⚠️  Setup required - choose one of the following options:"
echo ""
echo "📋 Option 1: Set capabilities on Node.js (Recommended)"
echo "   sudo setcap cap_net_raw+ep $NODE_PATH"
echo ""
echo "📋 Option 2: Enable unprivileged ping system-wide"
echo "   sudo sysctl -w net.ipv4.ping_group_range=\"0 2147483647\""
echo ""
echo "📋 Option 3: Run your application with sudo (Not recommended)"
echo "   sudo node your-app.js"
echo ""
echo "💡 For more details, see: https://github.com/luckyyyyy/node-ping-rs#permission-requirements"
