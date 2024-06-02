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
        { $capitalization ->
        [uppercase]
            { $show_value ->
            [true] A float literal {$value}
            *[false] A float literal
            }
        *[lowercase]
            { $show_value ->
            [true] a float literal {$value}
            *[false] a float literal
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
            [uppercase] No newline
            *[lowercase] no newline
            }
        *[false]
            { $capitalization ->
            [uppercase] Newline
            *[lowercase] newline
            }
        }
    [one]
        { $capitalization ->
        [uppercase]
            { $show_value ->
            [true] A new line {$value}
            *[false] A new line
            }
        *[lowercase]
            { $show_value ->
            [true] a new line {$value}
            *[false] a new line
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