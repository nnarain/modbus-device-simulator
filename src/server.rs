//
// server.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 26 2022
//
use crate::device::Device;

use std::{
    io,
    net::SocketAddr,
    pin::Pin,
};
use thiserror::Error;
use anyhow::Result;
use futures::future::Future;

use tokio_modbus::{
    prelude::*,
    server::{self, Service, NewService},
};
use tokio::sync::{mpsc, watch};

#[derive(Debug, Error)]
enum ServiceError {
    #[error("Failed to receive data from channel")]
    ReceiveError(#[from] watch::error::RecvError)
}

/// Handles spawning new service handlers for modbus clients
struct ServiceSpawner(mpsc::Sender<Request>, watch::Receiver<Response>);

impl NewService for ServiceSpawner {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Instance = DeviceService;

    fn new_service(&self) -> std::io::Result<Self::Instance> {
        // Create a new service with the request sender
        let tx = self.0.clone();
        let rx = self.1.clone();
        Ok(DeviceService::new(tx, rx))
    }
}

/// Handles dispatching requests to the virtual device
struct DeviceService {
    tx: mpsc::Sender<Request>,
    rx: watch::Receiver<Response>,
}

impl DeviceService {
    pub fn new(tx: mpsc::Sender<Request>, rx: watch::Receiver<Response>) -> Self {
        Self { tx, rx }
    }
}

impl Service for DeviceService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + Sync>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let tx = self.tx.clone();
        let mut rx = self.rx.clone();

        Box::pin(async move {
            // Send the request to the device task
            tx.send(req).await.unwrap();
            // Get the response from the device task
            // NOTE(nnarain): Watch is used here for single-producer:multi-consumer, but this might not be the right
            // approach for multi-request handling... Though that is not a use case here.
            let res = rx.changed().await;
            if res.is_ok() {
                let res = rx.borrow().clone();
                Ok(res)
            }
            else {
                // TODO(nnarain): clean up
                Err(io::Error::new(io::ErrorKind::Other, "Failed to receive from channel"))
            }
        })
    }
}

/// Main Modbus TCP server task
pub async fn run(sock_addr: SocketAddr, device: Device) -> Result<()> {
    // Setup the device task that actual handles the request from modbus clients
    // Requests are sent to the device task for handling
    // Responses are send back from the device task
    let (req_tx, req_rx) = mpsc::channel::<Request>(5);
    let (res_tx, res_rx) = watch::channel::<Response>(Response::Custom(0, vec![]));

    tokio::spawn(device_task(device, req_rx, res_tx));

    let spawner = ServiceSpawner(req_tx, res_rx);

    // Create a modbus tcp server and start with the service spawner
    let modbus_server = server::tcp::Server::new(sock_addr);
    modbus_server.serve(spawner).await?;

    Ok(())
}

/// Device task handles incoming requests from clients
async fn device_task(device: Device, mut rx: mpsc::Receiver<Request>, tx: watch::Sender<Response>) -> Result<()> {
    // Wait for incoming request
    while let Some(req) = rx.recv().await {
        // Use the virtual device to handle requests
        let res = match req {
            Request::ReadInputRegisters(addr, cnt) => {
                let regs = device.read_input_registers(addr, cnt).unwrap_or(vec![]);
                Response::ReadInputRegisters(regs)
            },
            Request::ReadDiscreteInputs(addr, cnt) => {
                let inputs = device.read_discrete_inputs(addr, cnt).unwrap_or(vec![]);
                Response::ReadDiscreteInputs(inputs)
            },
            Request::ReadCoils(addr, cnt) => {
                let coils = device.read_coils(addr, cnt).unwrap_or(vec![]);
                Response::ReadCoils(coils)
            },
            Request::ReadHoldingRegisters(addr, cnt) => {
                let regs = device.read_holding_registers(addr, cnt).unwrap_or(vec![]);
                Response::ReadHoldingRegisters(regs)
            }
            Request::WriteMultipleCoils(addr, coils) => {
                let (address, written) = device.write_coils(addr, coils).unwrap_or((addr, 0));
                Response::WriteMultipleCoils(address, written)
            },
            Request::WriteSingleCoil(address, value) => {
                let (address, _) = device.write_coils(address, vec![value]).unwrap_or((address, 0));
                Response::WriteSingleCoil(address, value)
            },
            Request::WriteMultipleRegisters(addr, values) => {
                let (address, written) = device.write_holding_registers(addr, values).unwrap_or((addr, 0));
                Response::WriteMultipleRegisters(address, written)
            }
            _ => unimplemented!()
        };

        tx.send(res)?;
    }

    Ok(())
}
