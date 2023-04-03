# Code Generation

This script can be used to generate code for interrupt assignments.

First create a file called `interrupts.csv` with the syntax `number,enum name,description`. An example can be seen below:

```csv
0,DUMMY0,dummy interrupt 0
1,DUMMY1,dummy interrupt 1
2,DUMMY2,dummy interrupt 2
3,DUMMY3,dummy interrupt 3
4,DUMMY4,dummy interrupt 4
5,DUMMY5,dummy interrupt 5
6,DUMMY6,dummy interrupt 6
7,DUMMY7,dummy interrupt 7
8,DUMMY8,dummy interrupt 8
9,DUMMY9,dummy interrupt 9
10,TIMER_LO,timer low interrupt
11,DUMMY11,dummy interrupt 11
12,DUMMY12,dummy interrupt 12
```

Then run the script with `python main.py`. It generates two files.

First `int_link.x` which must be placed in the top level directory of the PAC crate. It handles all linker script bindings.

Secondly it creates a file called `rust.rs`. Its content contain the interrupt enum definition. This definition should be used to replace the current definition inside the file `src/lib.rs`