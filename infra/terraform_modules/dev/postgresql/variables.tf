variable "container_name" {
  type = string
}

variable "user" {
  type    = string
  default = "postgres"
}

variable "password" {
  type = string
}

variable "default_database_name" {
  type    = string
  default = "postgres"
}

variable "host_port" {
  type = number
}
