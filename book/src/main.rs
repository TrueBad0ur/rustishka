extern crate iron;
use iron::prelude::*;
use iron::status;
use iron::mime::mime;
use iron::mime::__mime__ident_or_ext;

extern crate router;
use router::Router;

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>Sum Calculator</title>
        <form action="/sum" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute SUM</button>
        </form>
    "#);
    Ok(response)
}

fn summa(first: u64, second: u64)-> u64 {
    first + second
}