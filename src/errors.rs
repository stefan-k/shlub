// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
    foreign_links {
        // doesnt seem to work, check again
        IO(::std::io::Error);
    }
}
