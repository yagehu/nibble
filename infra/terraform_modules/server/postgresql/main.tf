terraform {
  required_providers {
    postgresql = {
      source  = "cyrilgdn/postgresql"
      version = "~> 1.18"
    }
  }
}

resource "postgresql_role" "main" {
  login          = true
  name           = var.user
  password       = var.password
  skip_drop_role = true
}

resource "postgresql_database" "main" {
  name       = var.database_name
  owner      = postgresql_role.main.name
  template   = "template0"
  encoding   = "UTF8"
  lc_collate = "en_US.UTF-8"
  lc_ctype   = "en_US.UTF-8"
}
