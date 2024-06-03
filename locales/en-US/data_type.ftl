
data-type-description-int8 = 
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No 8-bit integer
            *[lowercase] no 8-bit integer
            }
        *[false]
            { $capitalization ->
            [uppercase] 8-bit integer
            *[lowercase] 8-bit integer
            }
        }
    [one]
        { $show_count -> 
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] An 8-bit integer {$value}
                *[false] An 8-bit integer
                }
            *[lowercase]
                { $show_value ->
                [true] an 8-bit integer {$value}
                *[false] an 8-bit integer
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] 8-bit integer {$value}
                *[false] 8-bit integer
                }
            *[lowercase]
                { $show_value ->
                [true] 8-bit integer {$value}
                *[false] 8-bit integer
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} 8-bit integers
        *[false]
            { $capitalization ->
            [uppercase] 8-bit integers
            *[lowercase] 8-bit integers
            }
        }
    }

data-type-description-int16 = 
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No 16-bit integer
            *[lowercase] no 16-bit integer
            }
        *[false]
            { $capitalization ->
            [uppercase] 16-bit integer
            *[lowercase] 16-bit integer
            }
        }
    [one]
        { $show_count -> 
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A 16-bit integer {$value}
                *[false] A 16-bit integer
                }
            *[lowercase]
                { $show_value ->
                [true] a 16-bit integer {$value}
                *[false] a 16-bit integer
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] 16-bit integer {$value}
                *[false] 16-bit integer
                }
            *[lowercase]
                { $show_value ->
                [true] 16-bit integer {$value}
                *[false] 16-bit integer
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} 16-bit integers
        *[false]
            { $capitalization ->
            [uppercase] 16-bit integers
            *[lowercase] 16-bit integers
            }
        }
    }

data-type-description-int32 = 
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No 32-bit integer
            *[lowercase] no 32-bit integer
            }
        *[false]
            { $capitalization ->
            [uppercase] 32-bit integer
            *[lowercase] 32-bit integer
            }
        }
    [one]
        { $show_count -> 
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A 32-bit integer {$value}
                *[false] A 32-bit integer
                }
            *[lowercase]
                { $show_value ->
                [true] a 32-bit integer {$value}
                *[false] a 32-bit integer
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] 32-bit integer {$value}
                *[false] 32-bit integer
                }
            *[lowercase]
                { $show_value ->
                [true] 32-bit integer {$value}
                *[false] 32-bit integer
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} 32-bit integers
        *[false]
            { $capitalization ->
            [uppercase] 32-bit integers
            *[lowercase] 32-bit integers
            }
        }
    }

data-type-description-int = 
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No integer
            *[lowercase] no integer
            }
        *[false]
            { $capitalization ->
            [uppercase] Integer
            *[lowercase] integer
            }
        }
    [one]
        { $show_count -> 
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] An integer {$value}
                *[false] An integer
                }
            *[lowercase]
                { $show_value ->
                [true] an integer {$value}
                *[false] an integer
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Integer {$value}
                *[false] Integer
                }
            *[lowercase]
                { $show_value ->
                [true] integer {$value}
                *[false] integer
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} integers
        *[false]
            { $capitalization ->
            [uppercase] Integers
            *[lowercase] integers
            }
        }
    }

data-type-description-int64 = 
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No 64-bit integer
            *[lowercase] no 64-bit integer
            }
        *[false]
            { $capitalization ->
            [uppercase] 64-bit integer
            *[lowercase] 64-bit integer
            }
        }
    [one]
        { $show_count -> 
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A 64-bit integer {$value}
                *[false] A 64-bit integer
                }
            *[lowercase]
                { $show_value ->
                [true] a 64-bit integer {$value}
                *[false] a 64-bit integer
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] 64-bit integer {$value}
                *[false] 64-bit integer
                }
            *[lowercase]
                { $show_value ->
                [true] 64-bit integer {$value}
                *[false] 64-bit integer
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} 64-bit integers
        *[false]
            { $capitalization ->
            [uppercase] 64-bit integers
            *[lowercase] 64-bit integers
            }
        }
    }

data-type-description-uint8 =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No 8-bit unsigned integer
            *[lowercase] no 8-bit unsigned integer
            }
        *[false]
            { $capitalization ->
            [uppercase] 8-bit unsigned integer
            *[lowercase] 8-bit unsigned integer
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] An 8-bit unsigned integer {$value}
                *[false] An 8-bit unsigned integer
                }
            *[lowercase]
                { $show_value ->
                [true] an 8-bit unsigned integer {$value}
                *[false] an 8-bit unsigned integer
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] 8-bit unsigned integer {$value}
                *[false] 8-bit unsigned integer
                }
            *[lowercase]
                { $show_value ->
                [true] 8-bit unsigned integer {$value}
                *[false] 8-bit unsigned integer
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} 8-bit unsigned integers
        *[false]
            { $capitalization ->
            [uppercase] 8-bit unsigned integers
            *[lowercase] 8-bit unsigned integers
            }
        }   
    }

data-type-description-uint16 =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No 16-bit unsigned integer
            *[lowercase] no 16-bit unsigned integer
            }
        *[false]
            { $capitalization ->
            [uppercase] 16-bit unsigned integer
            *[lowercase] 16-bit unsigned integer
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A 16-bit unsigned integer {$value}
                *[false] A 16-bit unsigned integer
                }
            *[lowercase]
                { $show_value ->
                [true] a 16-bit unsigned integer {$value}
                *[false] a 16-bit unsigned integer
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] 16-bit unsigned integer {$value}
                *[false] 16-bit unsigned integer
                }
            *[lowercase]
                { $show_value ->
                [true] 16-bit unsigned integer {$value}
                *[false] 16-bit unsigned integer
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} 16-bit unsigned integers
        *[false]
            { $capitalization ->
            [uppercase] 16-bit unsigned integers
            *[lowercase] 16-bit unsigned integers
            }
        }
    }

data-type-description-uint32 = 
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No 32-bit unsigned integer
            *[lowercase] no 32-bit unsigned integer
            }
        *[false]
            { $capitalization ->
            [uppercase] 32-bit unsigned integer
            *[lowercase] 32-bit unsigned integer
            }
        }
    [one]
        { $show_count -> 
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A 32-bit unsigned integer {$value}
                *[false] A 32-bit unsigned integer
                }
            *[lowercase]
                { $show_value ->
                [true] a 32-bit unsigned integer {$value}
                *[false] a 32-bit unsigned integer
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] 32-bit unsigned integer {$value}
                *[false] 32-bit unsigned integer
                }
            *[lowercase]
                { $show_value ->
                [true] 32-bit unsigned integer {$value}
                *[false] 32-bit unsigned integer
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} 32-bit unsigned integers
        *[false]
            { $capitalization ->
            [uppercase] 32-bit unsigned integers
            *[lowercase] 32-bit unsigned integers
            }
        }
    }

data-type-description-uint = 
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No unsigned integer
            *[lowercase] no unsigned integer
            }
        *[false]
            { $capitalization ->
            [uppercase] Unsigned integer
            *[lowercase] unsigned integer
            }
        }
    [one]
        { $show_count -> 
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] An unsigned integer {$value}
                *[false] An unsigned integer
                }
            *[lowercase]
                { $show_value ->
                [true] an unsigned integer {$value}
                *[false] an unsigned integer
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Unsigned integer {$value}
                *[false] Unsigned integer
                }
            *[lowercase]
                { $show_value ->
                [true] unsigned integer {$value}
                *[false] unsigned integer
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} unsigned integers
        *[false]
            { $capitalization ->
            [uppercase] Unsigned integers
            *[lowercase] unsigned integers
            }
        }
    }

data-type-description-uint64 = 
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No 64-bit unsigned integer
            *[lowercase] no 64-bit unsigned integer
            }
        *[false]
            { $capitalization ->
            [uppercase] 64-bit unsigned integer
            *[lowercase] 64-bit unsigned integer
            }
        }
    [one]
        { $show_count -> 
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A 64-bit unsigned integer {$value}
                *[false] A 64-bit unsigned integer
                }
            *[lowercase]
                { $show_value ->
                [true] a 64-bit unsigned integer {$value}
                *[false] a 64-bit unsigned integer
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] 64-bit unsigned integer {$value}
                *[false] 64-bit unsigned integer
                }
            *[lowercase]
                { $show_value ->
                [true] 64-bit unsigned integer {$value}
                *[false] 64-bit unsigned integer
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} 64-bit unsigned integers
        *[false]
            { $capitalization ->
            [uppercase] 64-bit unsigned integers
            *[lowercase] 64-bit unsigned integers
            }
        }
    }

data-type-description-float32 =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No 32-bit floating point number
            *[lowercase] no 32-bit floating point number
            }
        *[false]
            { $capitalization ->
            [uppercase] 32-bit floating point number
            *[lowercase] 32-bit floating point number
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A 32-bit floating point number {$value}
                *[false] A 32-bit floating point number
                }
            *[lowercase]
                { $show_value ->
                [true] a 32-bit floating point number {$value}
                *[false] a 32-bit floating point number
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] 32-bit floating point number {$value}
                *[false] 32-bit floating point number
                }
            *[lowercase]
                { $show_value ->
                [true] 32-bit floating point number {$value}
                *[false] 32-bit floating point number
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} 32-bit floating point numbers
        *[false]
            { $capitalization ->
            [uppercase] 32-bit floating point numbers
            *[lowercase] 32-bit floating point numbers
            }
        }
    }

data-type-description-double =
    { $count ->
    [0] { $show_count ->
        [true]
            { $capitalization ->
            [uppercase] No double precision floating point number
            *[lowercase] no double precision floating point number
            }
        *[false]
            { $capitalization ->
            [uppercase] Double precision floating point number
            *[lowercase] double precision floating point number
            }
        }
    [one]
        { $show_count ->
        [true]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] A double precision floating point number {$value}
                *[false] A double precision floating point number
                }
            *[lowercase]
                { $show_value ->
                [true] a double precision floating point number {$value}
                *[false] a double precision floating point number
                }
            }
        *[false]
            { $capitalization ->
            [uppercase]
                { $show_value ->
                [true] Double precision floating point number {$value}
                *[false] Double precision floating point number
                }
            *[lowercase]
                { $show_value ->
                [true] double precision floating point number {$value}
                *[false] double precision floating point number
                }
            }
        }
    *[other]
        { $show_count ->
        [true] {$count} double precision floating point numbers
        *[false]
            { $capitalization ->
            [uppercase] Double precision floating point numbers
            *[lowercase] double precision floating point numbers
            }
        }
    }
