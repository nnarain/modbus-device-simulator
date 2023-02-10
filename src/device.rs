//
// device.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 26 2022
//

use mlua::{prelude::*, Function};
use thiserror::Error;

use std::io;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Failed to initialize device")]
    IoError(#[from] io::Error),
    #[error("Failed to load script")]
    ScriptError(#[from] LuaError),
}

/// Virtual device that uses a scripting runtime
pub struct Device {
    lua: Lua,
}

impl Device {
    pub fn new(script: &str) -> Result<Self, DeviceError> {
        // let script = fs::read_to_string(script)?;
        let lua = Lua::new();
        lua.load(script).exec().map_err(|e| DeviceError::ScriptError(e))?;

        Ok(Device {
            lua,
        })
    }

    /// Read input registers (read-only integer) from the virtual device
    pub fn read_input_registers(&self, address: u16, count: u16) -> Result<Vec<u16>, DeviceError> {
        let read_input_registers_fn: Function = self.lua.globals().get("ReadInputRegisters")?;
        let regs: Vec<u16> = read_input_registers_fn.call((address, count))?;

        Ok(regs)
    }

    /// Read discrete inputs (read-only boolean) from the virtual device
    pub fn read_discrete_inputs(&self, address: u16, count: u16) -> Result<Vec<bool>, DeviceError> {
        let read_discrete_inputs_fn: Function = self.lua.globals().get("ReadDiscreteInputs")?;
        let inputs: Vec<bool> = read_discrete_inputs_fn.call((address, count))?;

        Ok(inputs)
    }

    /// Read coils (read-write) from the virtual device
    pub fn read_coils(&self, address: u16, count: u16) -> Result<Vec<bool>, DeviceError> {
        let read_coils_fn: Function = self.lua.globals().get("ReadCoils")?;
        let coils: Vec<bool> = read_coils_fn.call((address, count))?;

        Ok(coils)
    }

    /// Write coils (read-write) from the virtual device
    pub fn write_coils(&self, address: u16, values: Vec<bool>) -> Result<(u16, u16), DeviceError> {
        let write_coils_fn: Function = self.lua.globals().get("WriteCoils")?;
        let res: Vec<u16> = write_coils_fn.call((address, values))?;

        Ok((res[0], res[1]))
    }

    /// Read holding registers (read-write) from the virtual device
    pub fn read_holding_registers(&self, address: u16, count: u16) -> Result<Vec<u16>, DeviceError> {
        let read_holding_registers_fn: Function = self.lua.globals().get("ReadHoldingRegisters")?;
        let regs: Vec<u16> = read_holding_registers_fn.call((address, count))?;
        Ok(regs)
    }

    /// Write holding registers (read-write) from the virtual device
    pub fn write_holding_registers(&self, address: u16, values: Vec<u16>) -> Result<(u16, u16), DeviceError> {
        let write_holding_registers_fn: Function = self.lua.globals().get("WriteHoldingRegisters")?;
        let res: Vec<u16> = write_holding_registers_fn.call((address, values))?;

        Ok((res[0], res[1]))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_device_from_script() {
        let script = r#"
            foo = 1
        "#;
        let device = Device::new(script);
        assert!(device.is_ok());
    }

    #[test]
    fn read_input_registers() {
        let script = r#"
            function ReadInputRegisters(addr, cnt)
                return {0, 1, 2}
            end
        "#;

        let device = Device::new(script).unwrap();
        let regs = device.read_input_registers(0, 10).unwrap();

        assert_eq!(regs, vec![0, 1, 2]);
    }

    #[test]
    fn read_discrete_inputs() {
        let script = r#"
            function ReadDiscreteInputs(addr, cnt)
                return {0, 1, 1}
            end
        "#;

        let device = Device::new(script).unwrap();
        let regs = device.read_discrete_inputs(0, 10).unwrap();

        assert_eq!(regs, vec![true, true, true]);
    }

    #[test]
    fn read_coils() {
        let script = r#"
            function ReadCoils(addr, cnt)
                return {false, true, true}
            end
        "#;

        let device = Device::new(script).unwrap();
        let regs = device.read_coils(0, 10).unwrap();

        assert_eq!(regs, vec![false, true, true]);
    }

    #[test]
    fn read_write_coils() {
        let script = r#"
            coils = {false, false, false}
            function WriteCoils(addr, values)
                for i = 1,#values do
                    coils[i] = values[i]
                end

                return {addr, #values}
            end
            function ReadCoils(addr, cnt)
                return coils
            end
        "#;

        let device = Device::new(script).unwrap();

        device.write_coils(0, vec![true, true, true]).ok().unwrap();
        let regs = device.read_coils(0, 10).unwrap();

        assert_eq!(regs, vec![true, true, true]);
    }

    #[test]
    fn read_holding_registers() {
        let script = r#"
            function ReadHoldingRegisters(addr, cnt)
                return {0, 1, 2, 3, 4}
            end
        "#;

        let device = Device::new(script).unwrap();
        let regs = device.read_holding_registers(0, 5).unwrap();

        assert_eq!(regs, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn read_write_holding_registers() {
        let script = r#"
            hr = {0, 0, 0}
            function WriteHoldingRegisters(addr, values)
                for i = 1,#values do
                    hr[i] = values[i]
                end

                return {addr, #values}
            end
            function ReadHoldingRegisters(addr, cnt)
                return hr
            end
        "#;

        let device = Device::new(script).unwrap();

        device.write_holding_registers(0, vec![0, 1, 2]).ok().unwrap();
        let regs = device.read_holding_registers(0, 3).unwrap();

        assert_eq!(regs, vec![0, 1, 2]);
    }

}
