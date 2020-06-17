#set debug remote 1
target extended-remote /dev/cu.usbmodemC1E3AACB1

set print asm-demangle on

# Default breakpoints to catch unhandled exceptions
#break DefaultHandler
#break HardFault
#break rust_begin_unwind

monitor tpwr enable
shell sleep 1
monitor swdp_scan
attach 1
set mem inaccessible-by-default off
monitor traceswo

#file target/thumbv7em-none-eabi/debug/examples/can
#file target/thumbv7em-none-eabihf/debug/examples/can
load

stepi
