#!/bin/bash

pac_check_libxcb=$(pacman -Q | grep -w "^libxcb")
if [$pac_check_libxcb == ""]
then
    echo "libxcb not found, installing..."
    sudo pacman -S libxcb
fi

pac_check_libxrandr=$(pacman -Q | grep -w "^libxrandr")
if [$pac_check_libxrandr == ""]
then
    echo "libxrandr not found, installing..."
    sudo pacman -S libxrandr
fi

pac_check_dbus=$(pacman -Q | grep -w "^dbus")
if [$pac_check_dbus=""]
then
    echo "dbus not found, installing..."
    sudo pacman -S dbus
fi

#Download models
DETECTION_MODEL="https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten"
RECOGNITION_MODEL="https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten"

echo "Downloading Models..."
curl "$DETECTION_MODEL" -o text-detection.rten
curl "$RECOGNITION_MODEL" -o text-recognition.rten