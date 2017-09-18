#[macro_use]
extern crate nom;

#[macro_use]
extern crate approx;

use nom::digit;
use std::str::FromStr;

fn main() {

    named!(signed_digits<&str,(Option<&str>,&str)>,
        pair!(
            opt!(alt!(tag!("+") | tag!("-"))),  // maybe sign?
            digit
        )
    );

    named!(maybe_signed_digits<&str,&str>,
        recognize!(signed_digits)
    );

    named!(floating_point<&str,&str>,
        recognize!(
            tuple!(
                maybe_signed_digits,
                opt!(complete!(pair!(
                    tag_s!("."),
                    digit
                ))),
                opt!(complete!(pair!(
                    alt!(tag_s!("e") | tag_s!("E")),
                    maybe_signed_digits
                )))
            )
        )
    );

    named!(float64<&str,f64>,
        map_res!(floating_point, FromStr::from_str)
    );

    named!(factor<&str,f64>,
        alt!(
            ws!(float64) |
            ws!(delimited!( tag_s!("("), expr, tag_s!(")") ))
        )
    );

    named!(term<&str,f64>, do_parse!(
        init: factor >>
        res: fold_many0!(
            tuple!(
                alt!(tag_s!("*") | tag_s!("/")),
                factor
            ),
            init,
            |acc, v:(_,f64)| {
                if v.0 == "*" {acc * v.1} else {acc / v.1}
            }
        )
        >> (res)
    ));

    named!(expr<&str,f64>, do_parse!(
        init: term >>
        res: fold_many0!(
            tuple!(
                alt!(tag_s!("+") | tag_s!("-")),
                term
            ),
            init,
            |acc, v:(_,f64)| {
                if v.0 == "+" {acc + v.1} else {acc - v.1}
            }
        )
        >> (res)
    ));

    macro_rules! expr_eq {
        ($e:expr) => (assert_relative_eq!(
            expr(stringify!($e)).to_result().unwrap(),
            $e)
        )
    }

    expr_eq!(2.3);
    expr_eq!(2.0 + 3.0 - 4.0);
    expr_eq!(2.0*3.0 - 4.0);

    expr_eq!(2.2*(1.1 + 4.5)/3.4);
    expr_eq!((1.0 + 2.0)*(3.0 + 4.0*(5.0 + 6.0)));



    named!(fold_sum<&str,f64>,
        fold_many1!(
            ws!(float64),
            0.0,
            |acc, v| acc + v
        )
    );

    println!("fold {:?}", fold_sum("1 2 3"));
    //fold 6

    assert_relative_eq!(fold_sum("1 2 3").to_result().unwrap(), 6.0);

}

