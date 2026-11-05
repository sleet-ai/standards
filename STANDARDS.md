# STANDARDS

general coding standards

- one function or stuct per a file
- files names should match function names
- clear_snake_case_names for functions ALL_SNAKE_CAPS for structs or consts.
- all things must end with what they are, `_fun` for functions, `_const` for consts, `_interface` for interfaces, matching the case of whatever, so an all caps struct `NETWORK_ID_STRUCT` stays all caps.
- `src/lib.rs` only pub mod in this file
- clean organized sub folder structure, like a fun, lib, or types folder, only main.rs and lib.rs should be in the src folder
- `// =====================` comments at the top after imports and at the bottom of file.
- every single one of those comments must be the exact same length everywhere in a project, about the length of the first line.
- fully typed and clean state managment is always perfered.
- less than 100 lines of code per file if we can, clear names, and usually one function per file.
- no `/* */` multi line comments in js, just use two `//`.
- for ts web apps i like bun, rsbuild and svelte.
- web apps should log cleanly to the console, data fetching and the data, but not for user navigation actions.
- console logs should have `======` on both sides, the same length as the comments everywhere else in that project.
- all the code in a project is consistent, and i try to be consistent across all projects.

=====================
<br/>
copyright 2026 by sleet.near
