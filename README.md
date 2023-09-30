# Change App Icon For Good

Change the icon of an app in MacOS for good, so that it won't break even after the app upgraded

# Solution

1. Use [fileicon](https://github.com/mklement0/fileicon) to change app's icon programaticlly
2. Compile the script to an executable so that it can be given permission to change other app's content
3. Register a Launch Daemon using `launchctl` to monitor a change in certain folder

# Dependency

We use [fileicon](https://github.com/mklement0/fileicon) to change the icon of apps, follow the instruction of the repository to install it.

# Usage

### Compile and execute the program to change all apps' icon
```shell
cargo build
sudo cargo run
```

### Produce launch daemon file
```shell
python src/produce_launch_daemon_file.py
```

### Register the launch deamon
```shell
sudo sh src/registerService.sh register
```

### Give the executable "Full Disk Access" permission
Go to the system setting, Security & Privacy, Full Disk Access. Add the compiled executable `target/debug/app_icon` to the permission list.

### Unregister the launch deamon (WIP)
```shell
sudo sh src/registerService.sh unregister
```

### Re-registered launch deamon (WIP)
```shell
sudo sh src/registerService.sh reregister
```

# TODO

1. 完整 registerService.sh、進入前檢測權限
2. 研究有沒有不用完全硬碟存取權限的方法
