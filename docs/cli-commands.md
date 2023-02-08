# CLI Commands

This documents describes CLI commands and operators for using RLPG.
Due to the current stage in the development process, only a few
commands will be supported in the beginning.

`--filename [PATH]`:
This is a required parameter.
It describes the location of the input file to generate the lexer and parser.

`--output [PATH]`:
This is a required parameter.
It describes where to put the output file (the generated lexer and parser) on the local storage.

`--namespace [STRING]`:
This is an optional parameter.
It describes the name of the namespace where the output code belongs.
This is beneficial if multiple files need to be generated so that namespace conflicts do not occur.
