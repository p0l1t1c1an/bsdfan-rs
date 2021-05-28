# bsdfan-rs
### A simple fan controller written in Rust for FreeBSD Thinkpads

This is a recreation of bsdfan, https://github.com/claudiozz/bsdfan, in the Rust language.

### Requirements
In order to utilize this program, Freebsd's Thinkpad ACPI drivers need to be loaded. 
You can compile the driver into your kernel by placing the following line in your kernel config:

`device acpi_ibm`

Alternatively, you can load the driver as a module at boot time, place the following line into your /boot/loader.conf:

`acpi_ibm_load="YES"`

### Installation
Clone this repository:
```
$ git clone https://github.com/p0l1t1c1an/bsdfan-rust.git
```
Then run make to compile the program:
```
$ make
```
Finally, run make install as root
```
# make install
```
This will install a binary called bsdfan, a config file as bsdfan.conf, and a startup script.

### Config
Located at /usr/local/etc/bsdfan.conf   
The configuration is the same as the original bsdfan, but with a delay setting.  
That means you can configure the fan levels as follows:
```
level(Number, Min_temp, Max_temp)

Number is the level number to be from 0 (idle) to 7 (full speed)
Min_temp is the temperature at which the program jumps back a level.
Max_temp is the temperature at which the program jumps up a level.

Level numbers must be in ascending order and at least 2 must be declared.
Each level must be declared on separate lines
The Min_temperature for the first level must be equal to 0.
The Max_temperature for the last level must be greater than 150, 
since that is what the original bsdfan has set as the max.
```
As well, you can configure the delay in the program:
```
delay Millis

Millis is the delay in milliseconds that program waits 
between checks of you computers temperature.
This config option is voluntary and the program will set the delay
speed to 2000 milliseconds, or 2 seconds.
```

This default config is installed unless a previous one already exists:
```
delay 3000
level (0,0,36)
level (1,34,40)
level (2,38,44)
level (3,42,48)
level (4,46,52)
level (5,50,56)
level (6,54,60)
level (7,58,68)
level (8,64,32767)
```

### Usage
To run the program call the following command as root: 
```
# bsdfan
```

To enable bsdfan-rs at startup include the following to your /etc/rc.conf:

`bsdfan_enable="YES"`


### Errors
`Need to write`

#### Acknowledgements
This program is heavily based off of bsdfan, https://github.com/claudiozz/bsdfan.   
Check out [@claudiozz](https://github.com/claudiozz) and their program.   
Let me know of usage and efficiency differences between theirs and mine.

#### Known Issues and Potential Improvements
- Improve verbosity of errors
  - Needs manual debug implementation for error types
  - Maybe have errors contain strings to print message
   
