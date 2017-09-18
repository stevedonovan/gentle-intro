#[macro_use]
extern crate nom;

named!(get_greeting<&str,&str>,
   ws!(alt!(tag_s!("hi") | tag_s!("bye")))
);

println!("{:?}", get_greeting(" hi "));
println!("{:?}", get_greeting(" bye "));
println!("{:?}", get_greeting("  hola "));

// Done("", "hi")
// Done("", "bye")
// Error(Alt)

named!(full_greeting<&str,(&str,&str)>,
    pair!(
        get_greeting,
        nom::alpha
    )
);

println!("result {:?}", full_greeting(" hi Bob  ").to_result());
// result Ok(("hi", "Bob"))

