import { useState } from "react";
import { useQuery, gql } from '@apollo/client';
import { styled } from '@mui/material/styles';
import Paper from '@mui/material/Paper';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell, { tableCellClasses } from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TablePagination from '@mui/material/TablePagination';
import TableRow from '@mui/material/TableRow';
import Collapse from '@mui/material/Collapse';
import IconButton from '@mui/material/IconButton';
import KeyboardArrowDownIcon from '@mui/icons-material/KeyboardArrowDown';
import KeyboardArrowUpIcon from '@mui/icons-material/KeyboardArrowUp';
import AvailabilityDialog from "../components/AvailabilityDialog";
import TableFallack from "../components/TableFallback";

const GET_MATRIX = gql`
query Matrix {
  matrix {
    categories {
      id
      name
    	products {
        id
        name
        availability {
            id
            jurisdiction {
                id 
            }
            lifecycle {id name}
			compliance {id name}
            lastUpdated
            comments {
                id
                text
                created
              }
            }
        features {
            id
            name
            availability {
              id
              jurisdiction {
                id 
              }
              lifecycle {id name}
			  compliance {id name}
              lastUpdated
              comments {
                id
                text
                created
              }
            }
        }
      }	
  	}
    compliances {id name}
    lifecycles {id name}
    jurisdictions { 
      id 
      name
      title
      cells {
        id
        name
        csp
        cspRegion
        country
        region
      }
    }
  }
}`;


const availability = (a) => {
    if (a == undefined) {
        return ''
    }
    return [a.lifecycle.name ?? '', a.compliance.name ?? ''].join(', ')
}

const jurisdictions = (jurisdictions, el, onClick) => {

    return jurisdictions.map(j => (
        <StyledTableCell key={j.id + ':' + el.id} onClick={onClick}>
            {availability(el.availability.find((a) => a.jurisdiction.id == j.id))}
        </StyledTableCell>
    ))

}

const matrix = (matrix, onClick) => {
    let spans = {};
    let rows = [];
    let jurisdiction_cols = matrix.jurisdictions.length;

    for (let category of matrix.categories) {
        let category_rows = 0
        for (let product of category.products) {
            let product_rows = product.features.length + 1;
            spans[product.id] = product_rows;
            category_rows += product_rows;
        }
        spans[category.id] = category_rows;
    }

    for (let category of matrix.categories) {
        for (let product of category.products) {
            rows.push(<StyledTableRow key={'row' + category.id + product.id + '_1'}>
                <StyledTableCell key={category.id + '_1'}> {category.name}</StyledTableCell>
                <StyledTableCell key={product.id + '_1'}>{product.name}</StyledTableCell>
                <StyledTableCell key={product.id + '_feature' + '_1'}></StyledTableCell>
                {[...jurisdictions(matrix.jurisdictions, product, onClick)]}
            </StyledTableRow>)
            for (let feature of product.features) {
                rows.push(<StyledTableRow key={'row' + category.id + product.id + feature.id + '_2'}>
                    <StyledTableCell key={category.id + '_2'}> {category.name}</StyledTableCell>
                    <StyledTableCell key={product.id + '_2'}>{product.name}</StyledTableCell>
                    <StyledTableCell key={feature.id + '_feature' + '_2'}>{feature.name}</StyledTableCell>
                    {[...jurisdictions(matrix.jurisdictions, feature, onClick)]}
                </StyledTableRow>)

            }
        }
    }

    return rows
}

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




export default function AvailabilityMatrix() {
    const [open, setOpen] = useState(false);
    const { loading, error, data } = useQuery(GET_MATRIX);

    const handleClickOpen = () => {
        setOpen(true);
    };

    const handleClose = () => {
        setOpen(false);
    };

    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error : {error.message}</p>;

    let jurisdictions = data.matrix.jurisdictions;


    if (loading) {
        return <TableFallack />
    }
    if (error) {
        return (
            <Paper sx={{ width: '100%', overflow: 'hidden', marginTop: '1em' }}>
                {error}
            </Paper>
        )
    }

    return (
        <>
            <Paper sx={{ width: '100%', overflow: 'hidden', height: 640, margin: '1em' }}>
                <TableContainer sx={{ width: '100%', maxHeight: 640 }
                } >
                    <Table stickyHeader aria-label="sticky table">
                        <TableHead>
                            <TableRow>
                                <StyledTableCell style={{ minWidth: 170 }} >{"Category"}</StyledTableCell>
                                <StyledTableCell style={{ minWidth: 170 }}  >{"Product (SKU)"}</StyledTableCell>
                                <StyledTableCell style={{ minWidth: 170 }}  >{"Feature"}</StyledTableCell>
                                {jurisdictions.map((jurisdiction) => (
                                    <StyledTableCell
                                        key={jurisdiction.id}
                                        style={{ minWidth: 100 }}
                                    >
                                        {jurisdiction.name}
                                    </StyledTableCell>
                                ))}
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {matrix(data.matrix, handleClickOpen)}
                        </TableBody>
                    </Table>
                </ TableContainer>
            </Paper>
            <AvailabilityDialog open={open} onClose={handleClose} />
        </>
    );
}