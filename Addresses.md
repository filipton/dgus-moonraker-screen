# Buttons (VP 0x1000):
 - 1 - ETA Pritning time (nav to 002)
 - 2 - Emergency (nav to 003)
 - 3 - Preheat (nav to 004)
 - 4 - Printer movement (nav to 005)
 - 5 - Settings (nav to XXX)
 - 6 - Back button (nav to 001)
 - 7 - Pause button in 002 
 - 8 - Stop button in 002
 - 9 - RESTART button from estop (nav to 001 - after printer restarts)
 - 10 - preheat PLA (200/45)
 - 11 - cooldown
 - 12 - reserved for custom temp
 - 13 - macros list UP
 - 14 - macros list down
 - 15 - macros button 1
 - 16 - macros button 2
 - 17 - macros button 3
 - 18 - macros button 4
 - 19 - Macros (nav to 006)

# Toolhead Movement Buttons (VP 0x1001)
 - 1 - y+
 - 2 - x+
 - 3 - x-
 - 4 - y-
 - 5 - z+
 - 6 - z-
 - 7 - home all

# Data Variables (text etc.) [ADDR/LEN]:
 - 0x2000/5 - "HH:MM" - header
 - 0x2005/10 - "ETA: HH:MM" - header
 - 0x2015/20 - "Model name" - 002
 - 0x2025/1 - current nozzle temp - 002
 - 0x2026/1 - target nozzle temp - 002
 - 0x2027/1 - current bed temp - 002
 - 0x2028/1 - target bed temp - 002
 - 0x2029/1 - printing progress bar (0-100)
 - 0x2030/1 - print paused (0 - unpaused, 1 - paused)

# List data vars (for example in macros) [ADDR/LEN]:
YES I KNOW THESE OFFSET ARE WILD BUT IT IS WHAT IT IS 
 - 0x3000/50 - macros list line 1
 - 0x3051/50 - macros list line 2
 - 0x3102/50 - macros list line 3
 - 0x3153/50 - macros list line 4
