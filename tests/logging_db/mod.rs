#[macro_use]
mod util;

#[test]
fn records_struct_and_trait() {
    logging_db_output_sufficient! {
        program {
            struct S {}

            trait Trait {}

            impl Trait for S {}
        }

        goal {
            S: Trait
        } yields {
            "Unique"
        }
    }
}

#[test]
#[ignore]
fn records_opaque_type() {
    logging_db_output_sufficient! {
        program {
            struct S {}

            trait Trait {}
            impl Trait for S {}

            opaque type Foo: Trait = S;
        }

        goal {
            Foo: Trait
        } yields {
            "Unique"
        }
    }
}

#[test]
#[ignore]
fn records_parents_parent() {
    logging_db_output_sufficient! {
        program {
            struct S {}

            trait Grandparent {}
            trait Parent where Self: Grandparent {}
            trait Child where Self: Parent {}
            impl Grandparent for S {}
            impl Parent for S {}
            impl Child for S {}
        }

        goal {
            S: Child
        } yields {
            "Unique"
        }
    }
}

#[test]
#[ignore]
fn records_associated_type_bounds() {
    logging_db_output_sufficient! {
        program {
            trait Foo {
                type Assoc: Bar;
            }
            trait Bar {

            }

            struct S {}
            impl Foo for S {
                type Assoc = S;
            }
            impl Bar for S {}
        }

        goal {
            S: Foo
        } yields {
            "Unique"
        }
    }
}

#[test]
fn records_generic_impls() {
    logging_db_output_sufficient! {
        program {
            struct S {}
            struct V {}

            trait Foo {}
            trait Bar {}

            impl Foo for S {}

            impl<T> Bar for T where T: Foo {

            }
        }

        goal {
            S: Bar
        } yields {
            "Unique"
        }
    }

    logging_db_output_sufficient! {
        program {
            struct S {}
            struct V {}

            trait Foo {}
            trait Bar {}

            impl Foo for S {}

            impl<T> Bar for T where T: Foo {

            }
        }

        goal {
            V: Bar
        } yields {
            "No possible solution"
        }
    }
}

#[test]
#[ignore]
fn records_necessary_separate_impl() {
    // might leave out the "impl Bar for Fox"
    logging_db_output_sufficient! {
        program {
            trait Box {
                type Assoc: Bar;
            }
            trait Bar {}

            struct Foo {}
            impl Box for Foo {
                type Assoc = Fox;
            }

            struct Fox {}
            impl Bar for Fox {}
        }

        goal {
            Foo: Box
        } yields {
            "Unique"
        }
    }
}
