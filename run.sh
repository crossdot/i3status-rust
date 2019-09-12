#!/usr/bin/env bash

cd ${0%/*}

MODE=${MODE:-release}
[[ ! -p /tmp/status.pipe ]] && mkfifo /tmp/status.pipe
[[ ! -p /tmp/status_in.pipe ]] && mkfifo /tmp/status_in.pipe
#OPENWEATHERMAP_API_KEY=e78e47e391f6ee4595469febe9ba6dca OPENWEATHERMAP_CITY_ID=498817
(tail -f /tmp/status_in.pipe) | target/${MODE}/i3status-rs config.toml > /tmp/status.pipe
