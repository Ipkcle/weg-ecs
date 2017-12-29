#[macro_export]
macro_rules! For {
    ($new_var:ident in $stream:expr => $block:block) => {
        loop {
            match $stream.next() {
                Some($new_var) => {
                    $block
                },
                None => break,
            }
        }
    };
}
