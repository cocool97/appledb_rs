import * as React from 'react';
import { Typography } from '@mui/material';

export const TitledComponent = ({ title, component }: { title: string, component: React.JSX.Element }) => (
    <>
        <Typography variant="h4" fontWeight="bold" marginBottom="2rem" marginLeft="2rem">{title}</Typography>
        {component}
    </>
)