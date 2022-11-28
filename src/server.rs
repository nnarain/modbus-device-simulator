//
// server.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 26 2022
//
use std::{
    net::SocketAddr,
    rc::Rc,
    sync::{Arc, Mutex},
    cell::RefCell, pin::Pin,
};
use anyhow::Result;
use futures::future;

use tokio_modbus::{
    prelude::*,
    server::{self, Service, NewService},
};

use crate::device::Device;

use mlua::prelude::*;

// impl NewService for DeviceServiceSpawner {
//     type Request = Request;
//     type Response = Response;
//     type Error = std::io::Error;
//     type Instance = DeviceService;

//     fn new_service(&self) -> std::io::Result<Self::Instance> {
//         Ok(DeviceService::new(self.backend.clone()))
//     }
// }

// impl Service for DeviceService {
//     type Request = Request;
//     type Response = Response;
//     type Error = std::io::Error;
//     type Future = future::Ready<Result<Self::Response, Self::Error>>;

//     fn call(&self, req: Self::Request) -> Self::Future {
//         match req {
//             Request::ReadInputRegisters(addr, cnt) => {
//                 let mut regs = vec![0; cnt.into()];
//                 future::ready(Ok(Response::ReadInputRegisters(regs)))
//             },
//             _ => {
//                 unimplemented!()
//             }
//         }
//     }
// }

// FIXME(nnarain): This is probably wrong...
// struct LuaWrapper(Lua);
// unsafe impl Send for LuaWrapper {}

struct Spawner(Arc<Mutex<Device>>);

impl NewService for Spawner {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Instance = MyService;

    fn new_service(&self) -> std::io::Result<Self::Instance> {
        let lua = self.0.clone();
        Ok(MyService(lua))
    }
}

struct MyService(Arc<Mutex<Device>>);

impl Service for MyService {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req {
            Request::ReadInputRegisters(addr, cnt) => {
                
            },
            _ => unimplemented!(),
        }
    }
}

pub async fn run(sock_addr: SocketAddr, device: Device) -> Result<()> {
    // let lua = LuaWrapper(Lua::new());
    let lua = Arc::new(Mutex::new(device));
    let spawner = Spawner(lua);

    // Create a modbus tcp server and start with the service spawner
    let modbus_server = server::tcp::Server::new(sock_addr);
    modbus_server.serve(spawner).await?;

    Ok(())
}
