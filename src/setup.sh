pac_check_libxcb=$(pacman -Q | grep -w "^libxcb")
if [$pac_check_libxcb == ""]
then
    echo "libxcb not found, installing..."
    sudo pacman -S libxcb
fi

pac_check_libxrandr=$(pacman -Q | grep -w "^libxrandr")
if [$pac_check_libxrandr == ""]
    echo "libxrandr not found, installing..."
    sudo pacman -S libxrandr
fi

pac_check_dbus=$(pacman -Q | grep -w "^dbus")
if [pac_check_dbus=""]
then
    echo "dbus not found, installing..."
    sudo pacman -S dbus
fi