#!/bin/bash

concurrently "npm start" "cargo run"
