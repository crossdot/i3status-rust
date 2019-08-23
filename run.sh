#!/usr/bin/env bash

cd ${0%/*}

mkfifo /tmp/status.pipe
#OPENWEATHERMAP_API_KEY=e78e47e391f6ee4595469febe9ba6dca OPENWEATHERMAP_CITY_ID=498817
target/release/i3status-rs config.toml > /tmp/status.pipe
