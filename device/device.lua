IR_ADDR = 44000

ir_block = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
discrete_inputs = {0, 1, 0, 1, 0, 1, 0, 1, 0, 1}
hr_block = {9, 8, 7, 6, 5, 4, 3, 2, 1, 0}
coils = {1, 0, 1, 0, 1, 0, 1}

function ReadInputRegisters(addr, cnt)
    return ir_block
end

function ReadDiscreteInputs(addr, cnt)
    return discrete_inputs
end
