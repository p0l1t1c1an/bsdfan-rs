# bsdfan-rust
### A simple fan controller written in Rust for FreeBSD Thinkpads

This is a recreation of bsdfan, https://github.com/claudiozz/bsdfan, in the memory safe Rust language.

### Requirements
In order to utilize this program, Freebsd's Thinkpad ACPI drivers need to be loaded. 
You can compile the driver into your kernel by placing the following line in your kernel config:
```
device acpi_ibm
```
Alternatively, you can load the driver as a module at boot time, place the following line into your /boot/loader.conf:
```
acpi_ibm_load="YES"
```

### Installation
Clone this repository:
```
# git clone https://github.com/p0l1t1c1an/bsdfan-rust.git
```
Then run cargo install as root:
```
# cargo install --path /path/to/bsdfan-rust
```

### Config
`Need to write`


### Errors
`Need to write`

#### Acknowledgements
This program is heavily based off of bsdfan, https://github.com/claudiozz/bsdfan. 
Check out [@claudiozz](https://github.com/claudiozz) and their program.   
Let me know of usage and efficiency differences between theirs and mine.

#### Known Issues and Potential Improvements
- Improved verbosity with errors
  - Needs manual debug implementation for error types
  - Maybe have errors contain strings to print message
  
- Binary installs to /usr/local/bin
  - The appropriate directory is /usr/local/sbin
  - This may be a limitation with cargo installation forcing a bin
   

