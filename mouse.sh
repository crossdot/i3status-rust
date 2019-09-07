#!/usr/bin/env bash

cd ${0%/*}

NAME=${NAME:-}
BUTTON=${BUTTON:-1}

event="{\"x\":0, \"y\":0, \"name\":\"${NAME}\", \"button\":${BUTTON}}"
echo $event > /tmp/status_in.pipe

exit 0
