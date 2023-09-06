# Pop-up menus and files
> src: http://www.cburch.com/logisim/docs/2.6.0/en/guide/mem/menu.html

The pop-up menu for memory includes four options in addition to the options common to all components:

* Edit Contents: Bring up a hex editor for editing the contents of memory.
* Clear Contents: Resets all values in memory to 0.
* Load Image...: Resets all values in memory based on the values found in a file using the format described below.
* Save Image...: Stores all values in memory into a file using the format described below.

The file format used for image files is intentionally simple; this permits you to write a program, such as an assembler, that generates memory images that can then be loaded into memory. As an example of this file format, if we had a 256-byte memory whose first five bytes were 2, 3, 0, 20, and -1, and all subsequent values were 0, then the image would be the following text file.

```
v2.0 raw
02
03
00
14
ff
```
The first line identifies the file format used (currently, there is only one file format recognized). Subsequent values list the values in hexadecimal, starting from address 0; you can place several such values on the same line. If there are more memory locations than are identified in the file, Logisim will load 0 into the other memory locations.

The image file can use run-length encoding; for example, rather than list the value 00 sixteen times in a row, the file can include 16*00. Notice than the number of repetitions is written in base 10. Files produced by Logisim will use run-length encoding for runs of at least four values.

You can place comments into the file by using the '#' symbol: All characters in the line starting from the '#' symbol will be ignored by Logisim.