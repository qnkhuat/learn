# Lifetime
- Lifetime annotation is used when a function take multiple references and return a reference
- This is needed because the compiled need to know what is the lifetime returned value. so you annotate it for both the input and output. so the output's lifetime can know which reference it's going to return
This complication is dued to the memory safety of Rust
