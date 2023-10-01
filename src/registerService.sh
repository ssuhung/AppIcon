#!/bin/bash -x

SCRIPTPATH=$(cd "$(dirname "$0")"; pwd -P)
SERVICE_NAME="com.ssuhung.appUpdateDetector"

if [ "$UID" -ne 0 ]; then
    echo "Superuser (root) permission is required to (un)register services"
    exit 1
fi

case "$1" in
  "register")
    chown root:wheel "${SCRIPTPATH}/${SERVICE_NAME}.plist"
    chmod 644 "${SCRIPTPATH}/${SERVICE_NAME}.plist"
    cp "${SCRIPTPATH}/${SERVICE_NAME}.plist" /Library/LaunchDaemons
    chmod 777 "${SCRIPTPATH}/${SERVICE_NAME}.plist"
    launchctl bootstrap system /Library/LaunchDaemons/${SERVICE_NAME}.plist
    launchctl enable system/${SERVICE_NAME}
    launchctl kickstart -kp system/${SERVICE_NAME}
    ;;
  # "reregister")
  #   chown root:wheel "${SCRIPTPATH}/com.ssuhung.appUpdateDetector.plist"
  #   chmod 600 "${SCRIPTPATH}/com.ssuhung.appUpdateDetector.plist"
  #   launchctl unload "${SCRIPTPATH}/com.ssuhung.appUpdateDetector.plist"
  #   launchctl load "${SCRIPTPATH}/com.ssuhung.appUpdateDetector.plist"
  #   chmod 777 "${SCRIPTPATH}/com.ssuhung.appUpdateDetector.plist"
  #   ;;
  "unregister")
    launchctl bootout system /Library/LaunchDaemons/${SERVICE_NAME}.plist
    ;;
  *)
    echo "Invalid command. Please specify a valid option (register, reregister, unregister)"
    exit 1
    ;;
esac
