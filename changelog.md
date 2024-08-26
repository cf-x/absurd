# Changelog

## 0.11.2

- added `update` command to update to the latest version
- updated cli
- fixed `attempt to subtract with overflow` error

## 0.11.1

- fixed methods for literals
- fixed method chaining
- fixed other method related bugs

## 0.11.0

- added methods for literals (`5.sqr()`) (works only on calls and isnt chained)
- added 44 new standard library functions
- side effects can no be disabled via CLI (`--side-effects` or `-s`)
- fixed negative numbers

## 0.10.4

- fixed manifest

## 0.10.3

- added support for loading standard library functions during runtime
- added support for function modality

## 0.10.2

- removed `project` category from manifest
- updated error handling
- fixed bugs

## 0.10.1

- reorganized project structure
- updated error handling system
- optimized most of the code
- reduced binary size
- increased execution speed

## 0.10.0

### core changes

- added `project.toml` manifest for configuration
- updated cli commands

### new settings

- `snippet` - change the snippet size in error messages
- `side_effects` - enables/disables side effects in project
- `disable_std` - disables the standard library
- `load_std` - enables/disables loading some standard library functions during runtime

## 0.9.1

- added array types
- added support for importing all values
- fixed mutability errors
- fixed error display
- fixed import of multiple values

## 0.9.0

- added modality
- added `mod` and `use` statements
- added variable publicity
- fixed parsing bugs
- fixed minor bugs

## 0.8.2

- updated error handling
- added emoji support in identifiers

## 0.8.1

- updated environment structure
- fixed assignments

## 0.8.0

- added assignments
- better error handling

## 0.7.3

- fixed function parameters
- fixed environmental vulnerability
- optimized environment handling

## 0.7.2

- fixed equality expressions
- fixed if statements

## 0.7.1

- fixed output printing twice
- fixed calling expressions

## 0.7.0

- added `array` expression
- added new standard library modules (`core::io`)
- fixed calls

## 0.6.1

- implemented error handling in interpreter
- added 6 new standard library modules

## 0.6.0

- updated `variable` and `function` interpreter
- added `if`, `while` and `loop` statement interpreter

## 0.5.1

- added cli `run` command
- minor optimizations

## 0.5.0

- implemented type checking
- implemented runtime error handling

## 0.4.0

- added expression interpreter
- updated literal handling
- fixed bugs

## 0.3.2

- updated standard library system
- fixed minor bugs

## 0.3.1

- fixed environmental vulnerability
- fixed interpreter

## 0.3.0

- updated interpreter
- updated expressions
- updated ast
- vulnerabilities detected

## 0.2.0

- added interpreter
- updated bundler
- fixed bugs

## 0.1.0

- added bundler modules
- updated ast
- fixed resolver errors
- fixed calls
- fixed tests
