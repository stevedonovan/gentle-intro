#[macro_use]
extern crate nom;

#[macro_use]
extern crate approx;

use nom::{IResult,alpha,digit};
use std::str::from_utf8;
use std::str::FromStr;

use std::fmt::Debug;

fn dump<T: Debug>(res: IResult<&[u8],T>) {
    match res {
      IResult::Done(bytes, value) => {println!("Done {:?} {:?}",from_utf8(bytes),value)},
      IResult::Error(err) => {println!("Err {:?}",err)},
      IResult::Incomplete(needed) => {println!("Needed {:?}",needed)}
    }
}

macro_rules! nom_res {
    ($p:expr,$t:expr) => ($p($t.as_bytes()).to_result())
}

fn main() {
    named!(get_greeting<&str>,
        ws!(map_res!(
            alt!( tag!("hi") | tag!("bye"))
        ,from_utf8))
    );

    dump(get_greeting(" hi ".as_bytes()));
    dump(get_greeting(" bye hi".as_bytes()));
    dump(get_greeting("  hola ".as_bytes()));

    println!("result {:?}", nom_res!(get_greeting, " bye  "));

    named!(full_greeting<(&str,Option<&str>)>,
        pair!(
            get_greeting,
            opt!(complete!(map_res!(alpha,from_utf8)))
        )
    );

    println!("result {:?}", nom_res!(full_greeting, " hi Bob  "));
    println!("result {:?}", nom_res!(full_greeting, " bye "));

    type IntResult<N> = Result<N,std::num::ParseIntError>;

    named!(digits<&str>,
        map_res!(digit,from_utf8)
    );

    named!(int8< IntResult<i8> >,
        map!(digits, FromStr::from_str)
    );

    println!("int {:?}", nom_res!(int8, "120"));
    println!("int {:?}", nom_res!(int8, "1200"));
    println!("int {:?}", nom_res!(int8, "x120"));

    named!(signed_digits<(Option<&[u8]>,&[u8])>,
        pair!(
            opt!(alt!(tag!("+") | tag!("-"))),  // maybe sign?
            digit
        )
    );

    println!("signed {:?}", nom_res!(signed_digits, "+12"));
    println!("signed {:?}", nom_res!(signed_digits, "4"));


    named!(maybe_signed_digits<&[u8]>,
        recognize!(signed_digits)
    );

    println!("signed {:?}", nom_res!(maybe_signed_digits, "+12"));


    named!(floating_point<&str>,
        map_res!(recognize!(
            tuple!(
                maybe_signed_digits,
                opt!(complete!(pair!(
                    tag!("."),
                    digits
                ))),
                opt!(complete!(pair!(
                    alt!(tag!("e") | tag!("E")),
                    maybe_signed_digits
                )))
            )
        ),from_utf8)
    );

    macro_rules! nom_eq {
        ($p:expr,$e:expr) => (assert_eq!(nom_res!($p, $e).unwrap(),$e))
    }

    nom_eq!(floating_point, "+2343");
    nom_eq!(floating_point, "-2343");
    nom_eq!(floating_point, "2343");
    nom_eq!(floating_point, "2343.23");
    nom_eq!(floating_point, "2e20");
    nom_eq!(floating_point, "2.0e-6");

    named!(float64<f64>,
        map_res!(floating_point, FromStr::from_str)
    );

    println!("got {}", nom_res!(float64,"2.3e-03").unwrap());

    //~ named!(pointf<(f64,&[u8],f64)>,
        //~ tuple!(
            //~ float64,
            //~ tag!(","),
            //~ float64
        //~ )
    //~ );

    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64
    }

    named!(pointf<Point>,
        do_parse!(
            first: float64 >>
            tag!(",") >>
            second: float64
            >>
            (Point{x: first, y: second})
        )
    );

    println!("got {:?}", nom_res!(pointf,"20,52.2").unwrap());

    named!(factor<f64>,
        alt!(
            ws!(float64) |
            ws!(delimited!( tag!("("), expr, tag!(")") ))
        )
    );

    named!(term<f64>, do_parse!(
        init: factor >>
        res: fold_many0!(
            tuple!(
                alt!(tag!("*") | tag!("/")),
                factor
            ),
            init,
            |acc, v:(_,f64)| {
                if v.0 == b"*" {acc * v.1} else {acc / v.1}
            }
        )
        >> (res)
    ));

    named!(expr<f64>, do_parse!(
        init: term >>
        res: fold_many0!(
            tuple!(
                alt!(tag!("+") | tag!("-")),
                term
            ),
            init,
            |acc, v:(_,f64)| {
                if v.0 == b"+" {acc + v.1} else {acc - v.1}
            }
        )
        >> (res)
    ));

    macro_rules! expr_eq {
        ($e:expr) => (assert_relative_eq!(
            nom_res!(expr,stringify!($e)).unwrap(),
            $e)
        )
    }

    expr_eq!(2.3);
    expr_eq!(2.0 + 3.0 - 4.0);
    expr_eq!(2.0*3.0 - 4.0);

    expr_eq!(2.2*(1.1 + 4.5)/3.4);
    expr_eq!((1.0 + 2.0)*(3.0 + 4.0*(5.0 + 6.0)));



    named!(fold_sum<f64>,
        fold_many1!(
            ws!(float64),
            0.0,
            |acc, v| acc + v
        )
    );

    println!("fold {}", nom_res!(fold_sum,"1 2 3").unwrap());
    //fold 6

    assert_relative_eq!(nom_res!(fold_sum,"1 2 3").unwrap(), 6.0);

}

