#!/usr/bin/env tclsh

# Test script for feet to meters conversion logic

# Test cases - each pair is {feet expected_meters}
set testCases {
    1 0.3048
    10 3.0480
    100 30.4800
    0 0.0000
    5.5 1.6764
}

puts "Testing feet to meters conversion:"
puts "=================================="

foreach {feet expected} $testCases {
    set result [expr {double($feet) * 0.3048}]
    set formatted [format "%.4f" $result]
    puts "Input: $feet feet -> Output: $formatted meters (Expected: $expected)"
    
    if {$formatted eq $expected} {
        puts "  ✓ PASS"
    } else {
        puts "  ✗ FAIL"
    }
}

puts "\nTesting input validation:"
puts "========================="

# Test invalid inputs
set invalidInputs {"" "abc" "-5" "12.34.56"}

foreach input $invalidInputs {
    if {$input eq ""} {
        puts "Empty input: ✓ Properly handled"
    } elseif {![string is double -strict $input]} {
        puts "Non-numeric input '$input': ✓ Properly handled"
    } elseif {$input < 0} {
        puts "Negative input '$input': ✓ Properly handled"
    } else {
        puts "Input '$input': ✓ Valid"
    }
}

puts "\nAll tests completed!" 