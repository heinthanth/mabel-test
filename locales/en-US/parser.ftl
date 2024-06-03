token-description-int = 
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No integer literal
            *[lowercase] no integer literal
            }
        *[false]
            { $capitalization ->
            [uppercase] Integer literal
            *[lowercase] integer literal
            }
        }
    [one]
        { $show_count -> 
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] An integer literal {$value}
                *[false] An integer literal
                }
            *[lowercase]
                { $show_value ->
                [true] an integer literal {$value}
                *[false] an integer literal
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Integer literal {$value}
                *[false] Integer literal
                }
            *[lowercase]
                { $show_value ->
                [true] integer literal {$value}
                *[false] integer literal
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} integer literals
        *[false]
            { $capitalization ->
            [uppercase] Integer literals
            *[lowercase] integer literals
            }
        }
    }

token-description-float = 
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No floating-point literal
            *[lowercase] no floating-point literal
            }
        *[false]
            { $capitalization ->
            [uppercase] Floating-point literal
            *[lowercase] floating-point literal
            }
        }
    [one]
        { $show_count -> 
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A floating-point literal {$value}
                *[false] A floating-point literal
                }
            *[lowercase]
                { $show_value ->
                [true] a floating-point literal {$value}
                *[false] a floating-point literal
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Floating-point literal {$value}
                *[false] Floating-point literal
                }
            *[lowercase]
                { $show_value ->
                [true] floating-point literal {$value}
                *[false] floating-point literal
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} floating-point literals
        *[false]
            { $capitalization ->
            [uppercase] Floating-point literals
            *[lowercase] floating-point literals
            }
        }
    }

token-description-identifier =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No identifier
            *[lowercase] no identifier
            }
        *[false]
            { $capitalization ->
            [uppercase] Identifier
            *[lowercase] identifier
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] An identifier {$value}
                *[false] An identifier
                }
            *[lowercase]
                { $show_value ->
                [true] an identifier {$value}
                *[false] an identifier
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Identifier {$value}
                *[false] Identifier
                }
            *[lowercase]
                { $show_value ->
                [true] identifier {$value}
                *[false] identifier
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} identifiers
        *[false]
            { $capitalization ->
            [uppercase] Identifiers
            *[lowercase] identifiers
            }
        }
    }

token-description-keyword =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No keyword
            *[lowercase] no keyword
            }
        *[false]
            { $capitalization ->
            [uppercase] Keyword
            *[lowercase] keyword
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A keyword {$value}
                *[false] A keyword
                }
            *[lowercase]
                { $show_value ->
                [true] a keyword {$value}
                *[false] a keyword
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Keyword {$value}
                *[false] Keyword
                }
            *[lowercase]
                { $show_value ->
                [true] keyword {$value}
                *[false] keyword
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} keywords
        *[false]
            { $capitalization ->
            [uppercase] Keywords
            *[lowercase] keywords
            }
        }
    }

token-description-operator =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No operator
            *[lowercase] no operator
            }
        *[false]
            { $capitalization ->
            [uppercase] Operator
            *[lowercase] operator
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] An operator {$value}
                *[false] An operator
                }
            *[lowercase]
                { $show_value ->
                [true] an operator {$value}
                *[false] an operator
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Operator {$value}
                *[false] Operator
                }
            *[lowercase]
                { $show_value ->
                [true] operator {$value}
                *[false] operator
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} operators
        *[false]
            { $capitalization ->
            [uppercase] Operators
            *[lowercase] operators
            }
        }
    }

token-description-colon =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No colon
            *[lowercase] no colon
            }
        *[false]
            { $capitalization ->
            [uppercase] Colon
            *[lowercase] colon
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A colon {$value}
                *[false] A colon
                }
            *[lowercase]
                { $show_value ->
                [true] a colon {$value}
                *[false] a colon
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Colon {$value}
                *[false] Colon
                }
            *[lowercase]
                { $show_value ->
                [true] colon {$value}
                *[false] colon
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} colons
        *[false]
            { $capitalization ->
            [uppercase] Colons
            *[lowercase] colons
            }
        }
    }

token-description-semicolon =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No semicolon
            *[lowercase] no semicolon
            }
        *[false]
            { $capitalization ->
            [uppercase] Semicolon
            *[lowercase] semicolon
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A semicolon {$value}
                *[false] A semicolon
                }
            *[lowercase]
                { $show_value ->
                [true] a semicolon {$value}
                *[false] a semicolon
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Semicolon {$value}
                *[false] Semicolon
                }
            *[lowercase]
                { $show_value ->
                [true] semicolon {$value}
                *[false] semicolon
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} semicolons
        *[false]
            { $capitalization ->
            [uppercase] Semicolons
            *[lowercase] semicolons
            }
        }
    }

token-description-left-paren =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No left parenthesis
            *[lowercase] no left parenthesis
            }
        *[false]
            { $capitalization ->
            [uppercase] Left parenthesis
            *[lowercase] left parenthesis
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A left parenthesis {$value}
                *[false] A left parenthesis
                }
            *[lowercase]
                { $show_value ->
                [true] a left parenthesis {$value}
                *[false] a left parenthesis
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Left parenthesis {$value}
                *[false] Left parenthesis
                }
            *[lowercase]
                { $show_value ->
                [true] left parenthesis {$value}
                *[false] left parenthesis
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} left parentheses
        *[false]
            { $capitalization ->
            [uppercase] Left parentheses
            *[lowercase] left parentheses
            }
        }
    }

token-description-right-paren =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No right parenthesis
            *[lowercase] no right parenthesis
            }
        *[false]
            { $capitalization ->
            [uppercase] Right parenthesis
            *[lowercase] right parenthesis
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A right parenthesis {$value}
                *[false] A right parenthesis
                }
            *[lowercase]
                { $show_value ->
                [true] a right parenthesis {$value}
                *[false] a right parenthesis
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Right parenthesis {$value}
                *[false] Right parenthesis
                }
            *[lowercase]
                { $show_value ->
                [true] right parenthesis {$value}
                *[false] right parenthesis
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} right parentheses
        *[false]
            { $capitalization ->
            [uppercase] Right parentheses
            *[lowercase] right parentheses
            }
        }
    }

token-description-single-line-comment =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No single-line comment
            *[lowercase] no single-line comment
            }
        *[false]
            { $capitalization ->
            [uppercase] Single-line comment
            *[lowercase] single-line comment
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A single-line comment {$value}
                *[false] A single-line comment
                }
            *[lowercase]
                { $show_value ->
                [true] a single-line comment {$value}
                *[false] a single-line comment
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Single-line comment {$value}
                *[false] Single-line comment
                }
            *[lowercase]
                { $show_value ->
                [true] single-line comment {$value}
                *[false] single-line comment
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} single-line comments
        *[false]
            { $capitalization ->
            [uppercase] Single-line comments
            *[lowercase] single-line comments
            }
        }
    }

token-description-whitespace =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No whitespace
            *[lowercase] no whitespace
            }
        *[false]
            { $capitalization ->
            [uppercase] Whitespace
            *[lowercase] whitespace
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A whitespace {$value}
                *[false] A whitespace
                }
            *[lowercase]
                { $show_value ->
                [true] a whitespace {$value}
                *[false] a whitespace
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Whitespace {$value}
                *[false] Whitespace
                }
            *[lowercase]
                { $show_value ->
                [true] whitespace {$value}
                *[false] whitespace
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} whitespaces
        *[false]
            { $capitalization ->
            [uppercase] Whitespaces
            *[lowercase] whitespaces
            }
        }
    }

token-description-tab =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No tab
            *[lowercase] no tab
            }
        *[false]
            { $capitalization ->
            [uppercase] Tab
            *[lowercase] tab
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A tab {$value}
                *[false] A tab
                }
            *[lowercase]
                { $show_value ->
                [true] a tab {$value}
                *[false] a tab
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Tab {$value}
                *[false] Tab
                }
            *[lowercase]
                { $show_value ->
                [true] tab {$value}
                *[false] tab
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} tabs
        *[false]
            { $capitalization ->
            [uppercase] Tabs
            *[lowercase] tabs
            }
        }
    }

token-description-newline =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No new line
            *[lowercase] no new line
            }
        *[false]
            { $capitalization ->
            [uppercase] New line
            *[lowercase] new line
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A new line {$value}
                *[false] A new line
                }
            *[lowercase]
                { $show_value ->
                [true] a newline {$value}
                *[false] a new line
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] New line {$value}
                *[false] New line
                }
            *[lowercase]
                { $show_value ->
                [true] new line {$value}
                *[false] new line
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} new lines
        *[false]
            { $capitalization ->
            [uppercase] New lines
            *[lowercase] new lines
            }
        }
    }

token-description-eoi =
    { $capitalization ->
    [uppercase]
        { $show_value ->
        [true] The end of input {$value}
        *[false] The end of input
        }
    *[lowercase]
        { $show_value ->
        [true] the end of input {$value}
        *[false] the end of input
        }
    }

lexer-error-unexpected-character = Unexpected character: {$character}
lexer-error-unimplemented-feature = Unimplemented feature: {$feature}
lexer-error-invalid-number-literal-width = Invalid width {$width} for {$literal_kind} literal. Possible widths are: {$valid_widths}
    .no-width-for-double = Double literals do not support width specification. They are always 64 bits wide.

