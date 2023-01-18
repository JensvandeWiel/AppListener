![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/JensvandeWiel/AppListener/test_push.yml?label=Tests&style=flat-square) ![GitHub release (latest by date)](https://img.shields.io/github/v/release/JensvandeWiel/AppListener?style=flat-square)

# App Listener
## What does this do?
App Listener is a simple project that checkes when a certain program is started and then starts another program and when the first program stops it stops the other program

## How to use

 1. Go to [Releases](https://github.com/JensvandeWiel/AppListener/releases/) and dowload the latest release.
 2. Unzip the folder to the location you want to store it.
 3. Copy the .env.example.com file to .env file and change it to your use case. (if you cannot see the .env.example file make sure to enable "Show hidden files" in file explorer)
 4. Start the program after reading the text below.
 
 We recommend letting the program start on boot.
### Windows
 1. Press the windows key and the R key at the same time.
 2. Enter `shell:startup` and press ok.
    
    ![enter image description here](https://supportkb.dell.com/img/ka06P000000HBwWQAW/ka06P000000HBwWQAW_nl_NL_3.jpeg) 
 3. Make a shortcut of the AppListener.exe and place it in this folder.
 4. Restart your pc to make AppListener start or manually start it.

### Status
Currently only windows is supported altough there is a linux binary too.

### Contribute
To help with this program create a fork and write some code. When done create a pull request on the main repo with clear documentation on what you have doen and why.
