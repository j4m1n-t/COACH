### Example Playbook (TOML)
    [playbook]
    name = "Example Playbook"

    [tasks]
    [tasks.task1]
    name = "Update and Upgrade"
    command = "sudo apt-get update && sudo apt-get upgrade -y"

    [tasks.task2]
    name = "Install NGINX"
    command = "sudo apt-get install nginx -y"

### Example Playbook (YAML)
    playbook:
    name: "Example Playbook"

    tasks:
    task1:
        name: "Update and Upgrade"
        command: "sudo apt-get update && sudo apt-get upgrade -y"

    task2:
        name: "Install NGINX"
        command: "sudo apt-get install nginx -y"

### Example Playbook (CUE)
    playbook: {
        name: "Example Playbook"

        tasks: {
            task1: {
                name: "Update and Upgrade"
                command: "sudo apt-get update && sudo apt-get upgrade -y"
            }

            task2: {
                name: "Install NGINX"
                command: "sudo apt-get install nginx -y"
            }
        }
    }