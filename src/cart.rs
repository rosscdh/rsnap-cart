use sapper::Result;
use sapper::SapperModule;
use sapper::Request;
use sapper::Response;
use sapper::SapperRouter;

use serde_json;

use std::str;

#[derive(Clone)]
pub struct Cart;

impl Cart {
    // those handlers in module Biz
    fn detail_cart(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, world!".to_string());
        
        Ok(response)
    }

    fn modify_cart(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, world!".to_string());
        
        Ok(response)
    }

    fn remove_cart(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, world!".to_string());
        
        Ok(response)
    }

    fn add_product(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, world!".to_string());
        
        Ok(response)
    }
}

// set before, after middleware, and add routers
impl SapperModule for Cart {
    
    // here add routers ....
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        // need to use Router struct here
        // XXX: here could not write as this, should record first, not parse it now
        
        
        router.get("/cart/:id", Cart::detail_cart);
        router.post("/cart/:id", Cart::modify_cart);
        router.delete("/cart/:id", Cart::remove_cart);
        router.get("/cart/:id/products", Cart::add_product);
        
        Ok(())
        
    }
    
    
}
