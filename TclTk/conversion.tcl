#!/usr/bin/env tclsh

# Conversion utilities for feet to meters
# This file can be sourced by other Tcl scripts

namespace eval conversion {
    # Conversion factor: 1 foot = 0.3048 meters
    variable CONVERSION_FACTOR 0.3048
    
    # Convert feet to meters with validation
    proc feet_to_meters {feet} {
        variable CONVERSION_FACTOR
        
        # Validate input
        if {$feet eq ""} {
            return -code error "Empty input provided"
        }
        
        if {![string is double -strict $feet]} {
            return -code error "Input must be a valid number"
        }
        
        if {$feet < 0} {
            return -code error "Input must be a positive number"
        }
        
        # Calculate conversion
        set result [expr {double($feet) * $CONVERSION_FACTOR}]
        
        # Return formatted result to 4 decimal places
        return [format "%.4f" $result]
    }
    
    # Validate feet input without conversion
    proc validate_feet {feet} {
        if {$feet eq ""} {
            return [list valid 0 message "Please enter a value"]
        }
        
        if {![string is double -strict $feet]} {
            return [list valid 0 message "Please enter a valid number"]
        }
        
        if {$feet < 0} {
            return [list valid 0 message "Please enter a positive number"]
        }
        
        return [list valid 1 message ""]
    }
    
    # Get conversion factor
    proc get_conversion_factor {} {
        variable CONVERSION_FACTOR
        return $CONVERSION_FACTOR
    }
    
    # Format result to specified decimal places
    proc format_result {value {decimals 4}} {
        return [format "%.${decimals}f" $value]
    }
    
    # Test the conversion with various inputs
    proc run_tests {} {
        puts "Running conversion tests..."
        puts "=========================="
        
        # Test valid conversions
        set testCases {
            1 0.3048
            10 3.0480
            100 30.4800
            0 0.0000
            5.5 1.6764
        }
        
        foreach {feet expected} $testCases {
            if {[catch {feet_to_meters $feet} result]} {
                puts "FAIL: Input $feet feet -> Error: $result"
            } else {
                puts "Input: $feet feet -> Output: $result meters (Expected: $expected)"
                if {$result eq $expected} {
                    puts "  ✓ PASS"
                } else {
                    puts "  ✗ FAIL"
                }
            }
        }
        
        # Test invalid inputs
        puts "\nTesting invalid inputs:"
        puts "======================"
        
        set invalidInputs {"" "abc" "-5" "12.34.56"}
        
        foreach input $invalidInputs {
            if {[catch {feet_to_meters $input} result]} {
                puts "✓ '$input' properly rejected: $result"
            } else {
                puts "✗ '$input' should have been rejected but returned: $result"
            }
        }
        
        puts "\nAll tests completed!"
    }
}

# If this file is run directly, execute tests
if {[info exists argv0] && [file tail [info script]] eq [file tail $argv0]} {
    conversion::run_tests
}
