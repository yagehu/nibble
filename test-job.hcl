job "test" {
    datacenters = ["default"]

    group "main" {
        count = 1

        task "main" {
            driver = "docker"

            config {
                image = ""
            }
        }
    }
}