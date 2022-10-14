#!/bin/bash

# Fail if any command fails.
set -e

# Install everthing needed to run tests, on top of what's needed for prod.

echo "Installing deps..."
apt install openjdk-11-jdk-headless python3 libssl-dev sudo -y
echo "Installing Firebase CLI..."
curl -sL https://firebase.tools | bash
