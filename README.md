# IBMfan
### A simple fan controller written Rust for FreeBSD w/ acpi_ibm

This is a fan controller that I created since FreeBSD doesn't control my fan automatically on my Thinkpad.  
I instead have to use `sysctl dev.acpi_ibm...` to manually configure the fan mode and fan speed.

Originally, I was planning on writing this program in C as I utilize a FreeBSD exclusive command `sysctlnametomib()`, which is said to 
be a bit of a hack, to convert the C name of a kernel state to its Management Information Base, an integer array. 
However, I have been learning Rust recently and decided to utilize that language instead. 

Currently, this program has the values for the fan to jump both up and down a level and the delay between checks are hardcoded into the program. 
I am planning on creating a config system to set those values.
As well, I plan on creating FreeBSD rc startup script, daemonizing the program, handing signals to exit, 
and I may plan on modifying the program to work with other fan controllers controlled by sysctl, if there are any.

A little after creating this program and getting it to work, I found out that there is a fan controller at https://github.com/claudiozz/bsdfan for Thinkpads.
For the time being, I would say to utilize that program first if you NEED a fan controller, but try mine out as well and let me know any issues with this program.
