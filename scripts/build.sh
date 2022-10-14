#!/bin/bash

# Build the project.
# Assumes all the dependencies to be already installed, see installProdDeps.sh.
# 
# Input env variables:
#   * CONFIG - "secretes" used to connect to Firebase backend. Should be a
#              string in JSON format. See Config struct in src/config.rs for
#              format.

echo $CONFIG > .env
trunk build
