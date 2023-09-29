#!/bin/bash -x

if command -v fileicon >/dev/null 2>&1 ; then
    SCRIPTPATH=$(cd "$(dirname "$0")"; pwd -P)
    APPFOLDER='/Applications/'
    APP_LIST=("Visual Studio Code" "The Unarchiver" "Microsoft Word" "Microsoft PowerPoint" "Microsoft Excel" "Microsoft OneNote" "zoom.us" "Google Chrome" "AppCleaner" "24 Hour Wallpaper")

    for APP_NAME in "${APP_LIST[@]}"
    do
        fileicon set "${APPFOLDER}${APP_NAME}.app" "${SCRIPTPATH}/../icon/${APP_NAME}.icns"
    done
else
    echo "fileicon is not installed, checkout https://github.com/mklement0/fileicon to install it"
fi
