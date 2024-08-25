import { useEffect, useState } from "react";
import { useQuery } from '@apollo/client';
import { GET_CELLS } from "../graph";

import { styled } from '@mui/material/styles';
import Paper from '@mui/material/Paper';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell, { tableCellClasses } from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Skeleton from '@mui/material/Skeleton';
import Stack from '@mui/material/Stack';


import { actor as cellActor } from '../state/cells';
import { Typography } from "@mui/material";

const columns = [

    { id: 'name', label: 'Cell Name', minWidth: 100 },
    { id: 'csp', label: 'CSP', minWidth: 100 },
    { id: 'country', label: 'Country', minWidth: 100 },
    { id: 'region', label: 'Region', minWidth: 100 },
    { id: 'cspRegion', label: 'CSP Region', minWidth: 100 },
];


const StyledTableCell = styled(TableCell)(({ theme }) => ({
    [`&.${tableCellClasses.head}`]: {
        backgroundColor: theme.palette.common.black,
        color: theme.palette.common.white,
    },
    [`&.${tableCellClasses.body}`]: {
        fontSize: 14,
    },
}));

const StyledTableRow = styled(TableRow)(({ theme }) => ({
    '&:nth-of-type(odd)': {
        backgroundColor: theme.palette.action.disabledBackground,
    },
    // hide last border
    '&:last-child td, &:last-child th': {
        border: 0,
    },
}));

const CellSkeleton = () => (
    <Stack spacing={1} >
        <Skeleton variant="rectangular" width={columns.length * (150 + 6)} height={24} />
        <Stack direction="row" spacing={1}>
            {columns.map((column) =>
                <Skeleton key={column.id} variant="rectangular" width={150} height={24} />
            )}
        </Stack>
        <Stack direction="row" spacing={1}>
            {columns.map((column) =>
                <Skeleton key={column.id} variant="rectangular" width={150} height={24} />
            )}
        </Stack>
        <Stack direction="row" spacing={1}>
            {columns.map((column) =>
                <Skeleton key={column.id} variant="rectangular" width={150} height={24} />
            )}
        </Stack>
    </Stack>
)



export default function Cells() {

    const { loading, error, data } = useQuery(GET_CELLS);


    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error : {error.message}</p>;


    return (
        <Paper sx={{ width: '100%', overflow: 'hidden', margin: '1em' }}>
            <Typography variant="h6">Cells</Typography>
            <TableContainer sx={{ maxHeight: 440 }}>
                <Table stickyHeader aria-label="sticky table">
                    <TableHead>
                        <TableRow>
                            {columns.map((column) => (
                                <StyledTableCell
                                    key={column.id}
                                    align={column.align}
                                    style={{ minWidth: column.minWidth }}
                                >
                                    {column.label}
                                </StyledTableCell>
                            ))}
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {data.cells.map((row) => {
                            return (
                                <StyledTableRow hover role="checkbox" tabIndex={-1} key={row.id}>
                                    {columns.map((column) => {
                                        const value = row[column.id];
                                        return (
                                            <StyledTableCell key={column.id} align={column.align}>
                                                {column.format && typeof value === 'number'
                                                    ? column.format(value)
                                                    : value}
                                            </StyledTableCell>
                                        );
                                    })}
                                </StyledTableRow>
                            );
                        })}
                    </TableBody>
                </Table>
            </TableContainer>
        </Paper>
    );
}