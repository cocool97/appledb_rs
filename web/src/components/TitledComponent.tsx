import * as React from "react";
import { Box, Typography } from "@mui/material";
import { InfoTooltip } from "./InfoTooltip";

export const TitledComponent = ({
  title,
  tooltip,
  logo,
  component,
}: {
  title: string;
  tooltip?: string;
  logo?: React.JSX.Element;
  component: React.JSX.Element;
}) => (
  <>
    <Box
      display="flex"
      width="inherit"
      marginBottom="2rem"
      marginLeft="2rem"
      alignItems="center"
      justifyContent="space-between"
    >
      <Box display="flex" alignItems="center">
        <Typography variant="h4" fontWeight="bold" marginRight="1rem">
          {title}
        </Typography>
        {tooltip && <InfoTooltip text={tooltip} />}
      </Box>
      {logo && (
        <Box display="flex" alignItems="center">
          {logo}
        </Box>
      )}
    </Box>
    {component}
  </>
);
