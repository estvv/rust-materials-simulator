#!/bin/bash
set -e

cd web
python3 -m http.server 8080
