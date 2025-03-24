import React from "react";
import DataTable from "react-data-table-component";
import "./CustomDataTable.css"

const columns = [
    {
        name: "Executable",
        selector: row => row.executable,
        sortable: false,
    }
];

const ExpandedRow = (props) => {
    const expandedColumns = [
        {
            name: "Entitlement key",
            selector: row => row.key,
            sortable: true,
        },
        {
            name: "Entitlement value",
            selector: row => row.value,
            sortable: true,
        },
    ];

    return (
        <div className="expanded-row">
            <DataTable
                columns={expandedColumns}
                data={props.data.data}
                noHeader
                highlightOnHover
                pointerOnHover
            />
        </div>
    );
};

const CustomDataTable = (props) => {
    const mainData = Object.entries(props.data).map(([executable, items]) => ({
        executable: `${executable} (${items.length})`,
        data: items,
    }));

    return (
        <DataTable
            title={"Entitlements per executables"}
            columns={columns}
            data={mainData}
            expandableRows
            expandableRowsComponent={ExpandedRow}
            highlightOnHover
            responsive
        />
    );
};

export default CustomDataTable;
