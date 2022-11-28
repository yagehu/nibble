terraform {
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 2.23"
    }

    postgresql = {
      source = "cyrilgdn/postgresql"
      version = "~> 1.18"
    }
  }
}

provider "docker" {
}

locals {
  config             = yamldecode(file("../../../src/server/tests/config.yaml"))
  database           = local.config.postgresql
}

module "postgresql_instance" {
  source = "../../terraform_modules/dev/postgresql"

  container_name = "nibble-dev-server-postgresql"
  user           = local.database.maintenance.user
  password       = local.database.maintenance.password
  host_port      = local.database.maintenance.port
}

provider "postgresql" {
  host     = local.database.maintenance.host
  port     = local.database.maintenance.port
  database = local.database.maintenance.database
  username = local.database.maintenance.user
  password = local.database.maintenance.password
  sslmode  = "disable"
}

module "postgres" {
  source = "../../terraform_modules/server/postgresql"

  database_name = local.database.main.database
  user          = local.database.main.user
  password      = local.database.main.password

  depends_on = [
    module.postgresql_instance
  ]
}

locals {
  num_nodes = 3
}

resource "docker_network" "main" {
  name   = "nibble-dev-main"
  driver = "bridge"
}

resource "docker_image" "dev" {
  name         = "nibble/dev:latest"
  keep_locally = true

  build {
    path       = "."
    dockerfile = "Containerfile"
  }
}

resource "docker_container" "nodes" {
  count = local.num_nodes

  name  = "nibble-dev-node-${count.index}"
  image = docker_image.dev.image_id

  volumes {
    container_path = "/consul/data"
  }

  volumes {
    container_path = "/nomad/data"
  }

  volumes {
    container_path = "/consul/config/server.json"
    host_path      = abspath("node_${count.index}.consul.server.json")
  }

  volumes {
    container_path = "/nomad/config/server.hcl"
    host_path      = abspath("nomad.server.hcl")
  }

  networks_advanced {
    name    = docker_network.main.name
    aliases = ["node_${count.index}"]
  }

  dynamic "ports" {
    for_each = count.index == 0 ? [true] : []

    content {
      internal = "8500"
      external = "8500"
    }
  }

  dynamic "ports" {
    for_each = count.index == 0 ? [true] : []

    content {
      internal = "4646"
      external = "4646"
    }
  }
}
