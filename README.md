# StampyLongTime

Small CLI program to help you modify timestamps. Mainly tested for Linux.

This tool exists because moving files between filesystems nukes your timestamps on Ext4 and most filesystems,
and by the time you realize it, your timestamps are already long gone.

Timestamps are very important, both for archival reasons and for organizational reasons.

### TODO:
- [ ] Implement a way to change the file creation date
    - Unsupported on most filesystems
    - Changing the system time, creating a file, and setting it back works as a scuffed workaround
    - ***Help needed***

### License
Currently using MIT. Planning to move to GPL3 in the future.
