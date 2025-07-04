<!--
Copyright lowRISC contributors.

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# parallel-cp

This project has been developed to meet lowRISC's internal need and is *not* supported by lowRISC.

This is a very simple tool that performs parallel local file copy.

It's aimed at speeding up copying when one of the locally mounted file system is actually over network and therefore has high latency and `cp` or `rsync` is using tiny fraction of your network bandwidth or disk IO.

Being a simple tool, it has minimal error handling -- use with care.

## Usage

`parallel-cp <SRC> <DST>`

Both `<SRC>` and `<DST>` should be directories.
They're not interpreted like `cp`: if you run `parallel-cp a b` and `b` is an existing directory, it does not copy `a` to `b/a`.
This operates like `cp --no-target-directory`.
Files nested under `<SRC>` with path `<SRC>/foo/bar` will be copied to `<DST>/foo/bar`.

By default this tool has parallel equal to the number of CPU cores.
For network file systems, this might be further increased to improve throughput.
You can set `RAYON_NUM_THREADS` to override the default.
