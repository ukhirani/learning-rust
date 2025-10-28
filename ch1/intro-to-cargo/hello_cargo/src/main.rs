fn main() {
    println!("Hello, world!");
}

//cargo build --> build the project and then you need to manually run it from the target folder
//there is also cargo build --release (much faster to execute and very optimized but slower to
//compile) --> so use debug build when repeatedly compiling

//cargo run   --> build and run the project all in the same command

//cargo check --> just the check that code compiles without generating a binary executable

// why would you not want an executable --> just because doing so is much slower than just
// compiling the code (cargo check is faster than cargo build)

//If you’re continually checking your work while writing the code, using cargo
//check will speed up the process!
