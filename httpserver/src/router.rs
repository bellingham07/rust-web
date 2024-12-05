use std::io::Write;
use http::httprequest;
use http::httprequest::{ HttpRequest, Resource};
use http::httpresponse::HttpResponse;
use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        println!("{:?}", req);
        match req.method {
            httprequest::HttpMethod::Get => match &req.resource {
                Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();
                    println!("route: {:?}", route);
                    match route[1] {
                        "api" => {
                            println!("api");
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            println!("________api____________");
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                println!("________apiqwe____________");

                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}