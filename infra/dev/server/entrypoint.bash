#!/usr/bin/env bash

set -euxo pipefail

/consul/consul agent -bootstrap-expect=3 -config-file=/consul/config/server.json &
/nomad/nomad agent -config=/nomad/config/server.hcl &

wait -n
exit $?
