import * as React from 'react';
import { Box, Typography } from '@mui/material';
import { InfoTooltip } from './InfoTooltip';

export const TitledComponent = ({ title, tooltip, component }: { title: string, tooltip?: string, component: React.JSX.Element }) => (
    <>
        <Box display="flex" width="inherit" marginBottom="2rem" marginLeft="2rem">
            <Typography variant="h4" fontWeight="bold" marginRight="1rem">{title}</Typography>
            {tooltip && <InfoTooltip text={tooltip} />}
        </Box>
        {component}
    </>
)