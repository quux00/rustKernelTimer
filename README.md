rustKernelTimer
===============

A simple and ugly kernel timer for *nix systems, implemented in Rust.

Requires pcwalton's zero.rs to work. Essentially, requests a timer interrupt at a very high frequency, and by so doing attempts to estimate an average kernel timer frequency, which it then outputs. This is very hacky and ugly, and was completed for an extra credit project. I plan to tidy it up at the least, if not improve its logic, in the near future.

Inspired by/cribs the strategy from http://www.advenage.com/topics/linux-timer-interrupt-frequency.php
