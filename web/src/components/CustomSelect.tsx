import { FormControl, InputLabel, MenuItem, Select } from "@mui/material";
import React from "react";

const CustomSelect = ({ onChange, choices, label, labelDisplayFunc, idGetter }) => (
    <FormControl fullWidth>
        <InputLabel sx={{ color: "white" }}>{label}</InputLabel>
        <Select
            defaultValue=""
            label={label}
            onChange={(event) => {
                onChange(event.target.value || null);
            }}
            sx={{ color: "white" }}
        >
            {choices.map((item) => (
                <MenuItem
                    key={idGetter(item)}
                    value={idGetter(item)}
                    sx={{ color: "black" }}
                >
                    {labelDisplayFunc(item)}
                </MenuItem>
            ))}
        </Select>
    </FormControl>
);

export default CustomSelect;
