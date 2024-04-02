pub const TOML: &str = r#"
unkown = "N/A" # Default value for unknown commands
background = [20, 15, 33, 0] # Background color as RGB value
topbar = true # true for bar at top of the screen, false for bar at bottom of the screen
height = 40 # Height of the bar

# Font settings

# Modules

# Modules are individual components of the bar that display different information. 
# Each module has a `command` which determines what information it displays, 
# and `x` and `y` values which determine its position on the bar.

# Workspaces Module

# This module displays the active and inactive workspaces. It takes two arguments: 
# the icon for the active window and the icon for the inactive window.

# Available for these compositors:
# - Hyprland

[[modules]]
x = 35.0
y = 20.0
command.Workspaces = [" ", " "]

# Battery Module

# This module displays the battery status. It takes three arguments: 
# the update time in milliseconds, the formatting for the display (with "%s" as a placeholder 
# for the value and %c as a placeholder for icons), and an array of icons.

[[modules]]
x = 1390.0
y = 20.0
command.Battery = [5000, "%c %s%", ["󰁺" ,"󰁺", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"]]

# Memory Module

# This module displays memory usage. It takes three arguments: 
# the memory option (e.g., "PercUsed" to display the percentage of memory used), 
# the update time in milliseconds, and the formatting for the display (with "%s" as a placeholder for the value).

[[modules]]
x = 1635.0
y = 20.0
command.Memory = ["PercUsed", 5000, "󰍛 %s%"]

# CPU Module

# This module displays CPU usage. It takes two arguments:  the update time in milliseconds, 
# and the formatting for the display (with "%s" as a placeholder for the value).

[[modules]]
x = 1700.0
y = 20.0
command.Cpu = [5000, " %s%"]

# Backlight Module

# This module is designed to show the level of screen backlight. It requires two arguments: 
# the display format (where "%s" is a placeholder for the value and "%c" is a placeholder for icons), and an array of icons.

[[modules]]
x = 1475.0
y = 20.0
command.Backlight = ["%c %s%", ["", "", "", "", "", "", "", "", ""]]

# Audio Module

# This module is designed to control and display the audio level. It takes two arguments: 
# the display format (where "%s" is a placeholder for the value and "%c" stands for icons), and an array of icons.

[[modules]]
x = 1540.0
y = 20.0
command.Audio = ["%c %s%", ["", "", "󰕾", ""]]

# Custom Module

# This module allows for custom commands. It takes three arguments: the command to execute, 
# the trigger event, and the formatting for the display (with "%s" as a placeholder for the value).

# Available trigger Events:

# WorkspaceChanged
# This event is triggered when the active workspace changes. It doesn't take any arguments.

# FileChanged
# This event is triggered when a specified file changes. It takes one argument: the path to the file to monitor for changes.

# TimePassed
# This event is triggered at regular intervals. It takes one argument: the time in milliseconds between updates.

# VolumeChanged
# This event is triggered when the volume changes. It doesn't take any arguments.

[[modules]]
x = 925.0
y = 20.0
command.Custom = ["date +%H:%M", { TimePassed = 60000 }, " %s"]

[[modules]]
x = 1775.0
y = 20.0
command.Custom = ["iwgetid -r", { TimePassed = 10000 }, "  %s"]
"#;

pub const CSS: &str = r#"
backlight {
    font-family: "JetBrainsMono Nerd Font";
    font-size: 16px;
    font-weight: bold;
    color: #ffffff;
}

battery {
    font-family: "JetBrainsMono Nerd Font";
    font-size: 16px;
    font-weight: bold;
    color: #ffffff;
}

audio {
    font-family: "JetBrainsMono Nerd Font";
    font-size: 16px;
    font-weight: bold;
    color: #ffffff;
}

cpu {
    font-family: "JetBrainsMono Nerd Font";
    font-size: 16px;
    font-weight: bold;
    color: #ffffff;
}

memory {
    font-family: "JetBrainsMono Nerd Font";
    font-size: 16px;
    font-weight: bold;
    color: #ffffff;
}

workspaces {
    font-family: "JetBrainsMono Nerd Font";
    font-size: 16px;
    font-weight: bold;
    color: #ffffff;
}

custom {
    font-family: "JetBrainsMono Nerd Font";
    font-size: 16px;
    font-weight: bold;
    color: #ffffff;
}
"#;
