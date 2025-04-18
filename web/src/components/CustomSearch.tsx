import { TextField } from "@mui/material";
import React from "react";

export const CustomSearch = ({ label, value, onChange }) => (
    <TextField
        label={label}
        variant="outlined"
        value={value}
        onChange={onChange}
        sx={{
            input: { color: "white" },
            label: { color: "white" },
            "& .MuiOutlinedInput-root": {
                color: "white",
                "& fieldset": {
                    borderColor: "white",
                },
                "&:hover fieldset": {
                    borderColor: "white",
                },
                "&.Mui-focused fieldset": {
                    borderColor: "white",
                },
                "&.Mui-disabled fieldset": {
                    borderColor: "black",
                },
                "& input": {
                    color: "white",
                },
                "& .MuiSvgIcon-root": {
                    color: "white",
                },
            },
            "& .MuiInputLabel-root": {
                color: "white",
                "&.Mui-focused": {
                    color: "white",
                },
                "&.Mui-disabled": {
                    color: "white",
                },
            },
        }}
    />
)