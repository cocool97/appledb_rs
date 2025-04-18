import { Tooltip } from "@mui/material";
import React from "react";
import InfoIcon from '@mui/icons-material/Info';


export const InfoTooltip = ({ text }) => (
    <Tooltip title={text} arrow sx={{ alignSelf: "center" }}>
        <InfoIcon style={{ color: "white" }} />
    </Tooltip>
)