#[macro_use]
extern crate lib;

item_element!(mod foo {});

struct Foo {
    opt: Option<usize>
}

fn main() {
    print_expr!(5);
    print_depending_on_number!();
    print_depending_on_number!(5);
    print_depending_on_number!(1, 2, 3);
    block_element!({
        let mut i = 5;
        i += 1;
        i
    });
    // statement_element!(let my_var = 5);

    let foo = Foo { opt: Some(5) };
    pattern_identifier_element!(foo, Foo {
        opt: Some(_)
    });
    type_element!(usize);
    path_element!(bar::baz::Baz);
    token_tree_element!(my_number, 42);
}

mod bar {
    pub mod baz {
        pub struct Baz;
    }
}
