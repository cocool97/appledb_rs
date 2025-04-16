import React, { useEffect, useState } from 'react';
import { API_URL } from '../Constants';
import {
    Card, Typography, CircularProgress, Table, TableBody, TableCell, TableContainer, TableRow
} from '@mui/material';

interface ServerStats {
    known_devices: number;
    known_operating_system_versions: number;
    known_executables: number;
    known_entitlements: number;
    known_frameworks: number;
}

const labelMap: Record<keyof ServerStats, string> = {
    known_devices: 'Known devices',
    known_operating_system_versions: 'Known OS versions',
    known_executables: 'Known executables',
    known_entitlements: 'Known entitlements',
    known_frameworks: 'Known frameworks',
};

const Stats = () => {
    const [stats, setStats] = useState<ServerStats | null>(null);
    const [errorMessage, setErrorMessage] = useState<string | null>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        fetch(`${API_URL}/stats`)
            .then((res) => res.json())
            .then((data: ServerStats) => {
                setStats(data);
                setLoading(false);
            })
            .catch((err) => {
                setErrorMessage(err.message);
                setLoading(false);
            });
    }, []);

    if (loading) {
        return (
            <Card sx={{ width: '100%', textAlign: 'center', p: 4 }}>
                <CircularProgress />
                <Typography color="white" sx={{ mt: 2 }}>Loading stats...</Typography>
            </Card>
        );
    }

    if (!stats) {
        return (
            <Typography color="white" textAlign="center">
                Failed to load stats: {errorMessage}
            </Typography>
        );
    }

    return (
        <TableContainer sx={{ display: "flex", justifyContent: "center" }}>
            <Table sx={{ height: "fit-content", width: "50%" }} >
                <TableBody>
                    {Object.entries(stats).map(([key, value]) => (
                        <TableRow
                            key={key}
                            sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
                        >
                            <TableCell sx={{ fontWeight: "bold", color: "white", width: "50%" }} align="left" component="th" scope="row">{labelMap[key as keyof ServerStats] || key}</TableCell>
                            <TableCell sx={{ fontWeight: "bold", color: "white", width: "50%" }} align="center">{value}</TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </TableContainer>
    );
};

export default Stats;
