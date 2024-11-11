#!/usr/bin/env bash
docker run -d --init --name signalk-server -p 3000:3000 -v /Users/andersar/.signalk:/home/node/.signalk signalk/signalk-server
