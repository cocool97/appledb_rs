import { TextField } from "@mui/material";
import React from "react";

interface CustomSearchProps {
    label: string;
    value: string;
    onChange: (event: React.ChangeEvent<HTMLInputElement>) => void;
    disabled?: boolean;
};

export const CustomSearch: React.FC<CustomSearchProps> = ({
    disabled = false,
    label,
    value,
    onChange,
}) => (
    <TextField
        disabled={disabled}
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
);
