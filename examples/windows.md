### Example Playbook(TOML)
    [playbook]
    name = "Example Playbook for Windows"

    [tasks]
    [tasks.task1]
    name = "Update Windows"
    command = "Start-Process powershell -ArgumentList 'Install-Module PSWindowsUpdate -Force' -Verb RunAs; Import-Module PSWindowsUpdate; Get-WindowsUpdate -Install -AcceptAll -AutoReboot"

    [tasks.task2]
    name = "Install Chocolatey"
    command = "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))"

    [tasks.task3]
    name = "Install Google Chrome"
    command = "choco install googlechrome -y"

### Example Playbook (YAML)
    playbook:
    name: "Example Playbook for Windows"

    tasks:
    task1:
        name: "Update Windows"
        command: >
        Start-Process powershell -ArgumentList 'Install-Module PSWindowsUpdate -Force' -Verb RunAs; 
        Import-Module PSWindowsUpdate; 
        Get-WindowsUpdate -Install -AcceptAll -AutoReboot

    task2:
        name: "Install Chocolatey"
        command: >
        Set-ExecutionPolicy Bypass -Scope Process -Force; 
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; 
        iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))

    task3:
        name: "Install Google Chrome"
        command: "choco install googlechrome -y"

### Example Playbook (CUE)
    playbook: {
        name: "Example Playbook for Windows"

        tasks: {
            task1: {
                name: "Update Windows"
                command: """
                Start-Process powershell -ArgumentList 'Install-Module PSWindowsUpdate -Force' -Verb RunAs; 
                Import-Module PSWindowsUpdate; 
                Get-WindowsUpdate -Install -AcceptAll -AutoReboot
                """
            }

            task2: {
                name: "Install Chocolatey"
                command: """
                Set-ExecutionPolicy Bypass -Scope Process -Force; 
                [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; 
                iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
                """
            }

            task3: {
                name: "Install Google Chrome"
                command: "choco install googlechrome -y"
            }
        }
    }