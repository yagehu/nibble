data_dir   = "/nomad/data"
datacenter = "default"

server {
  enabled          = true
  bootstrap_expect = 3
}

client {
  enabled = true
}

consul {
  address = "127.0.0.1:8500"
}
