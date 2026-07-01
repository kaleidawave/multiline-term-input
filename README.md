# multiline-term-input

A simple input library that handles `shift+enter`. If `mode == Mode::ReturnOnUndecoratedNewLine` then adds newline (same behavior as nodejs's REPL) else with `mode == Mode::ReturnOnShiftNewLine` continues until `shift+enter` is pressed.
