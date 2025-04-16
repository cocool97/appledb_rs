import React, { useState } from 'react';

import { TableContainer, Table, TableBody, TableCell, TableRow, TableHead, IconButton, Typography, Collapse, Box } from "@mui/material"
import { MdOutlineKeyboardArrowDown, MdOutlineKeyboardArrowUp } from "react-icons/md";

import "./CustomDataTable.css"

export const ExpandableTableRow = (
    { label, secondary, mainCellLabel, mainCellLabelGetter, secondaryCellLabel, secondaryCellLabelGetter, items }:
        { label: string, secondary?: string, mainCellLabel: string, mainCellLabelGetter: (item) => string, secondaryCellLabelGetter?: (item) => string, secondaryCellLabel?: string, items: [Record<string, any>] }
) => {
    const [open, setOpen] = useState(false);

    return (
        <>
            <TableRow>
                <TableCell width={"64px"} sx={{ textAlign: "center" }}>
                    <IconButton size="medium" onClick={items.length !== 0 ? () => setOpen(!open) : undefined}>
                        {open ? <MdOutlineKeyboardArrowUp color="white" /> : <MdOutlineKeyboardArrowDown color="white" />}
                    </IconButton>
                </TableCell>
                <TableCell width={"100%"}>
                    <div className="executable-name-cell">
                        <Typography>{label}</Typography>
                        <Typography>({items.length})</Typography>
                        {secondary && <Typography sx={{ fontStyle: "italic", fontSize: "0.8rem" }}>{secondary}</Typography>}
                    </div>
                </TableCell>
            </TableRow>

            {open && <TableRow>
                <TableCell colSpan={4} style={{ padding: 0 }}>
                    <Collapse in={open}>
                        <Box sx={{ display: "flex", justifyContent: "center", margin: 0 }}>
                            <Table size="small" sx={{ width: "100%" }}>
                                <TableHead>
                                    <TableRow>
                                        <TableCell sx={{ border: 0, minWidth: "64px" }} width={"64px"}></TableCell>
                                        <TableCell width={secondaryCellLabel ? "50%" : "100%"}><Typography variant="h5" sx={{ fontWeight: "bold", color: "white" }}>{mainCellLabel}</Typography></TableCell>
                                        {secondaryCellLabel && <TableCell width={"50%"}><Typography variant="h5" sx={{ fontWeight: "bold", color: "white" }}>{secondaryCellLabel}</Typography></TableCell>}
                                    </TableRow>
                                </TableHead>
                                <TableBody>
                                    {items.map((item) => (
                                        <TableRow key={item.id} className="table-cell-entitlement">
                                            <TableCell sx={{ border: 0, minWidth: "64px" }} width={"64px"}></TableCell>
                                            <TableCell width={secondaryCellLabel ? "50%" : "100%"} align="left" sx={{ fontWeight: "bold", color: "white" }}>{mainCellLabelGetter(item)}</TableCell>
                                            {secondaryCellLabel && <TableCell width={"50%"} align="left" sx={{ fontWeight: "bold", color: "white" }}>{secondaryCellLabelGetter(item)}</TableCell>}
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

export const CustomDataTable = (props) => {
    return (
        <TableContainer>
            <Table size="small" sx={{ tableLayout: "fixed" }}>
                <TableHead>
                    <TableRow>
                        <TableCell sx={{ width: "64px" }}></TableCell>
                        <TableCell sx={{ width: "100%" }}><Typography variant="h5" sx={{ fontWeight: "bold", color: "white" }}>Executables ({Object.keys(props.data).length})</Typography></TableCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {Object.entries(props.data).map(([executable_fullpath, { name, entitlements }]) => (
                        <ExpandableTableRow
                            key={executable_fullpath}
                            label={name}
                            mainCellLabel="Entitlement key"
                            mainCellLabelGetter={(item) => item.key}
                            secondaryCellLabel="Entitlement value"
                            secondaryCellLabelGetter={(item) => item.value}
                            secondary={executable_fullpath}
                            items={entitlements}
                        />
                    ))}
                </TableBody>
            </Table>
        </TableContainer>
    );
};