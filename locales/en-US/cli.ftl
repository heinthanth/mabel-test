cli-version =
    .heading = The Mabel Compiler
    .copyright = (c) { $copyrightYear } { $authors }
    .version = v{ $version }

cli-help =
    .usage-heading = usage:
    .about-heading = about:
    .subcommands-heading = subcommands:
    .options-heading = options:
    .positionals-heading = positionals:

    .arg-debug = Set debug level of the compiler/interpreter
    .arg-help = Show this help message and exit
    .arg-color = Set color output
    .arg-version = Show version and exit

cli-subcmd-compile-help =
    .description = Compile a program using specific backend
    .arg-input = Compile the program from <INPUT>
    .arg-backend = Compile the program using <BACKEND>
    .arg-output = Write the output to <OUTPUT>

cli-subcmd-run-help =
    .description = Run a program using the interpreter
    .arg-input = Run the program from <INPUT>
    .arg-no-jit = Disable JIT compilation

cli-error =
    .unexpected-subcommand = Unexpected subcommand { $subcommand }
    .unexpected-argument = Unexpected argument { $argument }
    .invalid-value = Invalid value { $value } for argument { $arg } and should be one of: { $choices }
    .arg-no-multiple-time = The argument { $arg } cannot be used multiple times
    .arg-conflict = The argument { $arg } conflicts with { $conflict }
    .generic = Something went wrong while parsing CLI arguments