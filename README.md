# Change App Icon For Good

Change the icon of an app in MacOS for good, so that it won't break even after the app upgraded

# Solution

1. Use [fileicon](https://github.com/mklement0/fileicon) to change app's icon programaticlly
2. Register a Launch Daemon using `launchctl` to monitor a change in certain folder

# Dependency

1. [fileicon](https://github.com/mklement0/fileicon)

# Usage

### Execute the script to change all app's icon
```shell
sudo sh changeFileIcon.sh
```

### Compile the script to executable
```shell
shc -f changeFileIcon.sh
```

### Produce launch daemon file
```shell
python src/produce_launch_daemon_file.py
```

### Register the launch deamon
```shell
sudo sh src/registerService.sh register
```

### Unregister the launch deamon
```shell
sudo sh src/registerService.sh unregister
```

### Re-registered launch deamon
```shell
sudo sh src/registerService.sh reregister
```

# TODO

1. 完整 registerService.sh、進入前檢測權限
2. 使用單一檔案管理存所有 app name
3. 不要使用 shc 編譯 script
