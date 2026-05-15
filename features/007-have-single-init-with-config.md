# 007 - Have single init() method with config

## Status: DONE

## Description

Update code of all drivers to have only one method `pub fn init(&mut self)` (remove `init_with_orientation()`). This method accept a config struct that allow config (e.g. `DisplayOn`, segment_remap, com_scan_reversed).

Try to have a common config.