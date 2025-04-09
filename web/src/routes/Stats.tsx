import React, { useEffect, useState } from 'react';
import { API_URL } from '../Constants';
import {
    TableContainer, Table, TableBody, TableCell, TableRow
} from "@mui/material"

const Stats = () => {
    const [stats, setStats] = useState({});

    useEffect(() => {
        fetch(`${API_URL}/stats`)
            .then((response) => response.json())
            .then((data) => setStats(data))
            .catch((error) => console.log(error));
    }, []);

    return (
        <TableContainer sx={{ display: "flex", justifyContent: "center" }}>
            <Table sx={{ width: "50%", height: "fit-content" }} >
                <TableBody>
                    {Object.entries(stats).map(([key, value]) => (
                        <TableRow
                            key={key}
                            sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
                        >
                            <TableCell sx={{ fontWeight: "bold", color: "white", width: "50%" }} align="left" component="th" scope="row">{key}</TableCell>
                            <TableCell sx={{ fontWeight: "bold", color: "white", width: "50%" }} align="center">{value}</TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </TableContainer>
    );
}

export default Stats