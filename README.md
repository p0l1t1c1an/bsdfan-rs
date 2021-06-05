# bsdfan-rs
### A simple fan controller written in Rust for FreeBSD Thinkpads

This is a recreation of [bsdfan](https://github.com/claudiozz/bsdfan) in the Rust language.

### Requirements
In order to utilize this program, Freebsd's Thinkpad ACPI drivers need to be loaded. 
You can compile the driver into your kernel by placing the following line in your kernel config:

`device acpi_ibm`

You can load the driver as a module at boot time by placing the following line into your /boot/loader.conf:

`acpi_ibm_load="YES"`

Or you can manually load it:

`# kldload acpi_ibm`


### Installation
Clone this repository:
```
$ git clone https://github.com/p0l1t1c1an/bsdfan-rs.git
```
cd into the cloned repo:
```
$ cd bsdfan-rs
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
Located at /usr/local/etc/bsdfan.toml    

#### Delay
There is an optional delay option in milliseconds  
Its default value is 2000 milliseconds (2 seconds)

Can be set like so:
```
delay = 3000
```

#### Levels
Levels is the set of all levels the fan will switch between.  
Each level has its number (num), min_temp (min), and max_temp (max). 

This is default config that is installed unless a previous one already exists:
```
Levels = [
{num = 0, min =  0, max = 44}, 
{num = 1, min = 40, max = 48},
{num = 2, min = 44, max = 52},
{num = 3, min = 48, max = 56},
{num = 4, min = 52, max = 60},
{num = 5, min = 56, max = 64},
{num = 6, min = 60, max = 68},
{num = 7, min = 64, max = 32767},
]
```

num (i32) is the fan_level from 0 (idle) to 7/8 (max speed).   
Some thinkpads support 7 as max and some have 8 as max.  
Safest to just set max to 7, if you don't know for your device.  

min (f32) is the temperature at which the program jumps back a level.  
max (f32) is the temperature at which the program jumps up a level.  

**Other Rules**  
You can NOT repeat nums in multiple levels set.  
The placement order of nums doesn't matter and your levels will be sort after being read.  
However, max of a level must be greater than the min of the next level once it has been sorted. 

#### Aliases
Aliases are also set is you want to change the variable names.

**delay** to delay_millis  
**Levels** to levels  
**num** to number, lvl, or level  
**min** to min_temp or minimum  
**max** to max_temp or maximum  


### Usage
To run the program call the following command as root: 
```
# bsdfan
```

To enable bsdfan-rs at startup include the following to your /etc/rc.conf:

`bsdfan_enable="YES"`


### Uninstallation
First, cd back into the cloned repo
```
$ cd /path/to/bsdfan-rs
```
Then, simply run make uninstall as root
```
# make uninstall
```
This will remove the binary and startup script.
Your config file will still exist and will have to be removed manually.  
You can do this by running rm on the config as root.
```
# rm /usr/local/etc/bsdfan.conf
```

### Acknowledgements
This program was somewhat based off of bsdfan, https://github.com/claudiozz/bsdfan.   
Check out [@claudiozz](https://github.com/claudiozz) and their programs.   
