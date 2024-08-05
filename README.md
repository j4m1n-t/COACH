# COACH - Comprehensive Orchestration And Configuration Hub
COACH is a powerful, flexible, and extensible tool designed for configuration management and orchestration. Inspired by Ansible, COACH allows you to define and execute configuration tasks using a variety of configuration formats, including TOML, YAML, and CUE. While COACH is primarily designed for Windows environments, it can also be used for Unix/Linux systems.

### Features

- <b>Multiple Configuration Formats</b>: Define your plays in TOML, YAML, or CUE.
- <b>CLI-Based</b>: Fully functional CLI tool with GUI development underway.
- <b>Versatile Protocol Support</b>: Execute tasks over SSH, HTTP, HTTPS, and WMI.
- <b>Windows Focused</b>: Optimized for Windows environments, with support for Unix/Linux systems.
- <b>Open Source and Community Driven</b>: Contributions from the developer community are highly encouraged.

### Getting Started
### Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

### Installation

Clone the repository:

    git clone https://github.com/yourusername/coach.git
    cd coach

Build the project:
    
    cargo build --release

Add the binary to your PATH:

    export PATH=$PATH:/path/to/coach/target/release

### Usage

To execute a playbook, use the following command:

    coach run -f /path/to/playbook.toml

COACH currently supports TOML, YAML, and CUE formats for playbooks. The format will be inferred from the file extension.

### Example Playbook (TOML)
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

For more examples see <a href="https://github.com/j4m1n-t/COACH/examples">examples</a>. 

### Contributing

COACH is in the early stages of development, and contributions are welcome! Here’s how you can help:
<ol>
<li>Fork the repository.</li>
<li>Create a new branch (git checkout -b feature/your-feature).</li>
<li>Make your changes.</li>
<li>Commit your changes (git commit -m 'Add some feature').</li>
<li>Push to the branch (git push origin feature/your-feature).</li>
<li>Open a Pull Request.</li>
</ol>

### Issues

If you encounter any issues, please report them in the <a href="https://github.com/j4m1n-t/COACH/issues">issue tracker</a>.

### Roadmap

- [ ] Complete GUI Development
- [ ] Add more modules for various configurations
- [ ] Improve documentation and add more examples
- [ ] Implement more security features
- [ ] Have fun

### License

This project is licensed under the MIT License. See the <a href="https://github.com/j4m1n-t/COACH/blob/master/LICENSE">LICENSE</a> file for details.

### Contact

For any questions or suggestions, feel free to reach out to the project maintainers or open an issue on GitHub.

---
Thank you for using COACH! Happy configuring!