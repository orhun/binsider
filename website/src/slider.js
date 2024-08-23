import { atom } from "nanostores";

export const index = atom(0);

import generalAnalysis from "./assets/demo/general-analysis.gif";
import staticAnalysis from "./assets/demo/static-analysis.gif";
import dynamicAnalysis from "./assets/demo/dynamic-analysis.gif";
import strings from "./assets/demo/strings.gif";
import hexdump from "./assets/demo/hexdump.gif";

export const images = atom([
  {
    src: generalAnalysis,
    alt: "General Analysis",
    text: '"Get inside of the ELF binaries."',
    description:
      "Binsider provides powerful tools for both static and dynamic analysis, offering features similar to readelf and strace. It allows you to easily inspect strings, examine linked libraries, and perform a hexdump, all within a user-friendly terminal user interface.",
  },
  {
    src: staticAnalysis,
    alt: "Static Analysis",
    text: '"Static analysis with a breeze."',
    description:
      "It helps you thoroughly examine the ELF file layout, including headers, sections, and symbols. It also supports precise searches across these components.",
  },
  {
    src: dynamicAnalysis,
    alt: "Dynamic Analysis",
    text: '"It is strace(1) but better."',
    description:
      "You can trace system calls and signals by running the executable. It also offers in-depth insights to further understand the program's behavior and interactions with the system.",
  },
  {
    src: strings,
    alt: "Strings",
    text: '"Strings are always interesting."',
    description:
      "It searches for printable character sequences in binaries for you and supports searching within them.",
  },
  {
    src: hexdump,
    alt: "Hexdump",
    text: '"Good ol\' hexdump."',
    description:
      "It provides detailed hexdump analysis, allowing you to view binary data in a readable hexadecimal format. You can also modify file data in hex or ASCII format.",
  },
]);
