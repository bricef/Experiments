#!/usr/bin/env tclsh

# Modern Tcl/Tk Feet to Meters Converter
# Compatible with Tcl 9.0+

package require Tk 9.0

# Source the conversion module
source [file join [file dirname [info script]] conversion.tcl]

# Initialize global variables
set ::feet ""
set ::meters ""

# Create main window
wm title . "Feet to Meters Converter"
wm geometry . "400x200"
wm resizable . 0 0

# Create main frame with proper padding
set mainFrame [ttk::frame .main -padding "20 20 20 20"]
pack $mainFrame -fill both -expand true

# Configure grid weights for responsive layout
grid columnconfigure $mainFrame 1 -weight 1
grid rowconfigure $mainFrame 0 -weight 1
grid rowconfigure $mainFrame 1 -weight 1
grid rowconfigure $mainFrame 2 -weight 1

# Create and place widgets
ttk::label $mainFrame.title -text "Feet to Meters Converter" -font "TkDefaultFont 14 bold"
grid $mainFrame.title -column 0 -row 0 -columnspan 3 -pady "0 20"

# Input section
ttk::label $mainFrame.feetLabel -text "Feet:"
grid $mainFrame.feetLabel -column 0 -row 1 -sticky e -padx "0 10"

ttk::entry $mainFrame.feetEntry -width 15 -textvariable ::feet -font "TkDefaultFont 12"
grid $mainFrame.feetEntry -column 1 -row 1 -sticky ew -padx "0 10"

ttk::button $mainFrame.calcButton -text "Convert" -command calculate -width 10
grid $mainFrame.calcButton -column 2 -row 1 -sticky w

# Output section
ttk::label $mainFrame.resultLabel -text "Meters:"
grid $mainFrame.resultLabel -column 0 -row 2 -sticky e -padx "0 10"

ttk::label $mainFrame.metersLabel -textvariable ::meters -font "TkDefaultFont 12" 
grid $mainFrame.metersLabel -column 1 -row 2 -sticky w -padx "0 10"

# Status label for errors
ttk::label $mainFrame.statusLabel -text "" 
grid $mainFrame.statusLabel -column 0 -row 3 -columnspan 3 -pady "10 0"

# Bind Enter key to calculate function
bind $mainFrame.feetEntry <Return> {calculate}

# Focus on entry widget
focus $mainFrame.feetEntry

# Calculate function using the conversion module
proc calculate {} {
    global feet meters
    
    # Clear previous status
    clearStatus
    
    # Use the conversion module for validation and conversion
    if {[catch {conversion::feet_to_meters $feet} result]} {
        set ::meters ""
        showStatus $result red
    } else {
        set ::meters $result
        showStatus "Conversion completed successfully!" green
    }
}

# Clear status message
proc clearStatus {} {
    .main.statusLabel configure -text ""
}

# Show status message
proc showStatus {message {color red}} {
    .main.statusLabel configure -text $message -foreground $color
}

# Main event loop
if {[info exists tk_library]} {
    # Running with Tk
    wm protocol . WM_DELETE_WINDOW {exit}
} else {
    # Running without Tk (shouldn't happen with this script)
    puts "This script requires Tk to run"
    exit 1
}
