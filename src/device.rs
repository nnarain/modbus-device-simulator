//
// device.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 26 2022
//

use mlua::{prelude::*, Function};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Failed to load script")]
    ScriptLoadError,
}

/// Virtual device that uses a scripting runtime
pub struct Device {
    lua: Lua,
}
// FIXME(nnarain): This is probably wrong...
unsafe impl Send for Device {}

impl Device {
    pub fn new(script: &str) -> Result<Self, DeviceError> {
        let lua = Lua::new();
        lua.load(script).exec().map_err(|_| DeviceError::ScriptLoadError)?;

        Ok(Device {
            lua,
        })
    }

    // pub fn read_input_registers(&self, address: u16, count: u16) -> Result<Vec<u16>, DeviceError> {
    //     Ok(vec![0; count as usize])
    // }
}
