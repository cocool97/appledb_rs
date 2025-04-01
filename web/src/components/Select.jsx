import { FormControl, InputLabel, Select, MenuItem } from "@mui/material";

const CustomSelect = (props) => {

    const { onChange, choices, label, labelDisplayFunc, idGetter } = props;

    return <FormControl fullWidth>
        <InputLabel sx={{ color: "white" }}>{label}</InputLabel>
        <Select
            label={label}
            onChange={(event, newValue) => {
                onChange(newValue.props.value || null);
            }}
            sx={{ color: "white" }}
        >
            {choices.map((item) => {
                return <MenuItem sx={{ input: { backgroundColor: "transparent", color: "white" }, color: "black" }} key={item} value={idGetter(item)}>{labelDisplayFunc(item)}</MenuItem>
            })}
        </Select>
    </FormControl>
}

export default CustomSelect;