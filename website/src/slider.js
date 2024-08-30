import { atom } from "nanostores";

export const index = atom(0);

import generalAnalysis from "./assets/demo/binsider-general-analysis.gif";
import staticAnalysis from "./assets/demo/binsider-static-analysis.gif";
import dynamicAnalysis from "./assets/demo/binsider-dynamic-analysis.gif";
import strings from "./assets/demo/binsider-strings.gif";
import hexdump from "./assets/demo/binsider-hexdump.gif";

export const images = atom([
  {
    src: generalAnalysis,
    alt: "General Analysis",
    text: '"Get inside of the ELF binaries."',
    description:
      "Binsider offers powerful static and dynamic analysis tools, similar to readelf(1) and strace(1). It lets you inspect strings, examine linked libraries, and perform hexdumps, all within a user-friendly TUI.",
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
      "It searches for sequences of printable characters in binaries and supports searching within those sequences for discovering specific patterns or data.",
  },
  {
    src: hexdump,
    alt: "Hexdump",
    text: '"Good ol\' hexdump."',
    description:
      "It provides detailed hexdump analysis, allowing you to view and modify binary data in a readable hexadecimal or ASCII format.",
  },
]);
