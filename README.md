# `pulp-device`

This crate acts as the PAC (peripheral access crate) for the controlPULP IP.

## Features

It does provide the structs to add custom device peripherals. However, currently none are implemented.

It however provides an Enum of interrupts available on the controlPULP IP. It currently mostly consists of dummy interrupts, but meaningful names could be added easily.
Additionally it provides the necessary linker script to connect the interrupt enum names to the CLIC vector table. For more specific details see the README of the runtime crate.

## Code Generation

To generate the enum list, as well as the linker script, a python script exists in the folder `code_generation`. It takes a CSV file as input and generates the necessary code snippets. Like this, the description of all interrupts can come from a single CSV file.