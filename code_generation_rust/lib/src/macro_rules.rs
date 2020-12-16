/// Print the value of a given expression.
/// Also shows that macros can be called from macros.
#[macro_export]
macro_rules! print_expr {
    ($e:expr) => { println!("{}", $e) }
}

/// Show usage of different element numbers
#[macro_export]
macro_rules! print_depending_on_number {
    () => {
        println!("Nothing :(")
    };

    ($e:expr) => {
        println!("One value: {}", $e)
    };

    ( $($e:expr),*) => {
        println!("Multiple Elements:");
        $(
            println!("{}", $e);
        )*
    };
}

/// A macro accepting an item, writing the cfg(test) attribute over it
/// Items are https://doc.rust-lang.org/stable/reference/items.html
/// Examples: module definitions, function declarations, trait declarations, etc.
#[macro_export]
macro_rules! item_element {
    ($i:item) => {
        #[cfg(test)]
        $i
    };
}

/// A macro accepting a block (bunch of statements in braces).
/// Prints the value of the block
#[macro_export]
macro_rules! block_element {
    ($b:block) => {
        println!("Value of the block: {}", $b)
    }
}

/// A macro accepting a statement (item, let statement, expression statement or macro invocation);
/// The goal  was to declare a variable my_var with a statement and print it. However, this doesn't seem to work.
#[macro_export]
macro_rules! statement_element {
    ($s:stmt) => {
        $s;
        println!("{}", my_var)
    }
}

/// A macro accepting a identifier (variable name) and a pattern. If the pattern matches
/// the value of the given identifier, print a success message.
#[macro_export]
macro_rules! pattern_identifier_element {
    ($i:ident, $p:pat) => {
        if let $p = $i {
            println!("Pattern matches!")
        } else {
            println!("Pattern does not match!")
        }
    }
}

/// A macro accepting a type. Prints the default value of the type.
/// Angle brackets are required here, I don't know why.
#[macro_export]
macro_rules! type_element {
    ($t:ty) => {
        let default_value = <$t>::default();
        println!("Default value: {}", default_value)
    }
}

/// A macro accepting a typepath. Generates a use.
#[macro_export]
macro_rules! path_element {
    ($p:path) => {
        use $p;
    }
}

#[macro_export]
macro_rules! token_tree_element {
    ($name:tt, $value:tt) => {
        let $name: usize = $value;
    }
}