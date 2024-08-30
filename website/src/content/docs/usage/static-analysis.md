---
title: Static Analysis
sidebar:
  order: 2
---

![layout](../../assets/static.jpg)

:::tip

To select different blocks in this layout and scroll them vertically, you can use the <kbd>n</kbd>/<kbd>p</kbd> (next/previous) and <kbd>h</kbd>/<kbd>j</kbd>/<kbd>k</kbd>/<kbd>l</kbd> keys. Also, press <kbd>enter</kbd> to view the details of the selected item.

:::

---

ELF files consist of different sections, segments, and headers, each containing information about the binary file. Static analysis is the process of examining these parts to better understand the file's structure **without** executing it.

:::note[ELF Overview]

Here is a diagram from [this blog post](https://scratchpad.avikdas.com/elf-explanation/elf-explanation.html) which shows the structure of an ELF file:

![ELF overview](https://scratchpad.avikdas.com/elf-explanation/file-overview.svg)

:::

---

### File Headers

This portion of the layout shows the ELF file headers similar to the output of `readelf -h <binary>` command. It includes the following information:

![file headers](../../assets/file-headers.jpg)

This might come in handy when you want to understand the binary's architecture and the entry point.

---

### Notes

Here you can see the notes found in the binary file, similar to the output of `readelf -n <binary>` command.

![notes](../../assets/notes.jpg)

This information is useful for understanding the binary's build environment and the compiler version used.

---

### Common Sections Table

This table shows the common sections found in the binary file, similar to the output of `readelf -S <binary>` command. It includes the following information:

| **Section**     | **Description**                                          |
| --------------- | -------------------------------------------------------- |
| Program headers | Segments loaded into memory when the binary is executed. |
| Section headers | Sections storing the binary's data.                      |
| Symbols         | Contains symbols used in the binary.                     |
| Dynamic symbols | Contains dynamic symbols used in the binary.             |
| Dynamic section | Contains dynamic linking information.                    |
| Relocations     | Contains relocations used in the binary.                 |

You can press <kbd>h</kbd> and <kbd>l</kbd> to scroll horizontally and <kbd>/</kbd> to search for a specific value.

![static table](../../assets/static-table.gif)
