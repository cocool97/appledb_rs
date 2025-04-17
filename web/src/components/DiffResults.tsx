import React from "react"
import { Table, TableBody, TableContainer } from "@mui/material";
import { ExpandableTableRow } from "./CustomDataTable";
import { Diff } from "../types/diff";



const DiffResults = (
    { diff, mainCellLabel, mainCellLabelGetter, secondaryCellLabel, secondaryCellLabelGetter }:
        { diff: Diff | null, mainCellLabel, mainCellLabelGetter, secondaryCellLabel?, secondaryCellLabelGetter?}) => (
    <div style={{ display: "flex" }}>
        {
            diff && (
                <TableContainer>
                    <Table size="small" sx={{ tableLayout: "fixed" }}>
                        <TableBody>
                            <ExpandableTableRow
                                label="Added"
                                mainCellLabel={mainCellLabel}
                                mainCellLabelGetter={mainCellLabelGetter}
                                secondaryCellLabel={secondaryCellLabel}
                                secondaryCellLabelGetter={secondaryCellLabelGetter}
                                items={diff.added}
                            />
                            <ExpandableTableRow
                                label="Removed"
                                mainCellLabel={mainCellLabel}
                                mainCellLabelGetter={mainCellLabelGetter}
                                secondaryCellLabel={secondaryCellLabel}
                                secondaryCellLabelGetter={secondaryCellLabelGetter}
                                items={diff.removed}
                            />
                            <ExpandableTableRow
                                label="Unchanged"
                                mainCellLabel={mainCellLabel}
                                mainCellLabelGetter={mainCellLabelGetter}
                                secondaryCellLabel={secondaryCellLabel}
                                secondaryCellLabelGetter={secondaryCellLabelGetter}
                                items={diff.unchanged}
                            />
                        </TableBody>
                    </Table>
                </TableContainer>
            )
        }
    </div>
)

export default DiffResults;