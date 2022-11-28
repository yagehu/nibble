terraform {
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 2.23"
    }
  }
}

resource "docker_image" "postgresql" {
  name = "postgres:15-alpine"
}

resource "docker_container" "main" {
  name         = var.container_name
  image        = docker_image.postgresql.image_id
  wait         = true
  wait_timeout = 10

  env = [
    "POSTGRES_USER=${ var.user }",
    "POSTGRES_PASSWORD=${ var.password }",
    "POSTGRES_DB=${ var.default_database_name }",
  ]

  healthcheck {
    interval     = "1s"
    retries      = 2
    start_period = "1s"
    timeout      = "1s"
    test = [
      "CMD",
      "pg_isready",
      "--host=localhost",
      "--port=5432",
      "--dbname=postgres",
      "--username=postgres",
    ]
  }

  volumes {
    container_path = "/var/lib/postgresql/data"
  }

  ports {
    internal = "5432"
    external = var.host_port
  }
}
