import React, { useState } from 'react';

import { TableContainer, Table, TableBody, TableCell, TableRow, TableHead, IconButton, Typography, Collapse, Box } from "@mui/material"
import { MdOutlineKeyboardArrowDown, MdOutlineKeyboardArrowUp } from "react-icons/md";

import "./CustomDataTable.css"

const ExpandableRow = ({ executable, executable_fullpath, items }) => {
    const [open, setOpen] = useState(false);

    return (
        <>
            <TableRow>
                <TableCell width={"64px"} sx={{ textAlign: "center" }}>
                    <IconButton size="medium" onClick={() => setOpen(!open)}>
                        {open ? <MdOutlineKeyboardArrowUp color="white" /> : <MdOutlineKeyboardArrowDown color="white" />}
                    </IconButton>
                </TableCell>
                <TableCell width={"33%"}>
                    <div className="executable-name-cell">
                        <Typography>{executable}</Typography>
                        <Typography>({items.length})</Typography>
                        <Typography sx={{ fontStyle: "italic", fontSize: "0.8rem" }}>{executable_fullpath}</Typography>
                    </div>
                </TableCell>
                <TableCell width={"33%"}></TableCell>
                <TableCell width={"33%"}></TableCell>
            </TableRow>

            {open && <TableRow>
                <TableCell colSpan={4} style={{ padding: 0 }}>
                    <Collapse in={open}>
                        <Box sx={{ display: "flex", justifyContent: "center", margin: 0 }}>
                            <Table size="small" sx={{ width: "100%" }}>
                                <TableHead>
                                    <TableRow>
                                        <TableCell sx={{ border: 0 }} width={"64px"}></TableCell>
                                        <TableCell sx={{ border: 0 }} width={"33%"}></TableCell>
                                        <TableCell width={"33%"}><Typography variant="h5" sx={{ fontWeight: "bold", color: "white" }}>Entitlement name</Typography></TableCell>
                                        <TableCell width={"33%"}><Typography variant="h5" sx={{ fontWeight: "bold", color: "white" }}>Entitlement value</Typography></TableCell>
                                    </TableRow>
                                </TableHead>
                                <TableBody>
                                    {items.map((item) => (
                                        <TableRow key={item.id} className="table-cell-entitlement">
                                            <TableCell sx={{ border: 0 }} width={"64px"}></TableCell>
                                            <TableCell sx={{ border: 0 }} width={"33%"}></TableCell>
                                            <TableCell width={"33%"} align="left" sx={{ fontWeight: "bold", color: "white" }}>{item.key}</TableCell>
                                            <TableCell width={"33%"} align="left" sx={{ fontWeight: "bold", color: "white" }}>{item.value}</TableCell>
                                        </TableRow>
                                    ))}
                                </TableBody>
                            </Table>
                        </Box>
                    </Collapse>
                </TableCell>
            </TableRow>}
        </>
    );
};

const CustomDataTable = (props) => {
    return (
        <TableContainer>
            <Table size="small" sx={{ tableLayout: "fixed" }}>
                <TableHead>
                    <TableRow>
                        <TableCell sx={{ width: "64px" }}></TableCell>
                        <TableCell sx={{ width: "100%" }}><Typography variant="h5" sx={{ fontWeight: "bold", color: "white" }}>Executables ({Object.keys(props.data).length})</Typography></TableCell>
                        {/* <TableCell sx={{ width: "33%" }}></TableCell>
                        <TableCell sx={{ width: "33%" }}></TableCell> */}
                    </TableRow>
                </TableHead>
                <TableBody>
                    {Object.entries(props.data).map(([executable_fullpath, { name, entitlements }]) => (
                        <ExpandableRow key={executable_fullpath} executable={name} executable_fullpath={executable_fullpath} items={entitlements} />
                    ))}
                </TableBody>
            </Table>
        </TableContainer>
    );
};

export default CustomDataTable;