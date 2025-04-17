import { Autocomplete, TextField } from "@mui/material";
import React from "react";

const CustomAutocomplete = (props) => {
    const { disabled, options, onChange, inputLabel } = props;

    console.log(inputLabel)

    return (
        <Autocomplete
            disabled={disabled}
            fullWidth
            disablePortal
            sx={{
                "& .MuiOutlinedInput-root": {
                    color: "white",
                    "& fieldset": {
                        borderColor: disabled ? "black" : "white",
                    },
                    "&:hover fieldset": {
                        borderColor: disabled ? "black" : "white",
                    },
                    "&.Mui-focused fieldset": {
                        borderColor: disabled ? "black" : "white",
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
                "& + .MuiAutocomplete-popper .MuiAutocomplete-option": {
                    backgroundColor: "transparent",
                    color: "black",
                },
                "& + .MuiAutocomplete-popper .MuiAutocomplete-option[aria-selected='true']": {
                    backgroundColor: "transparent",
                    color: "black",
                },
                "& + .MuiAutocomplete-popper .MuiAutocomplete-option[aria-selected='true'].Mui-focused": {
                    backgroundColor: "transparent",
                    color: "black",
                },
            }}
            options={options}
            renderInput={(params) => <TextField sx={{ label: { color: 'white' }, input: { color: "white !important" } }} {...params} label={inputLabel} />}
            onChange={onChange}
        />
    );
};

export default CustomAutocomplete;
