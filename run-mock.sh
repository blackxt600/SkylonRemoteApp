#!/bin/bash

# Script to run the elliptical server in mock mode
# Useful for development on macOS or systems without Bluetooth hardware

echo "🔧 Starting Elliptical Server in MOCK mode..."
echo ""
echo "This mode simulates the Kettler bike without Bluetooth hardware."
echo "Perfect for frontend development and testing!"
echo ""

cargo run --release --no-default-features --features mock
