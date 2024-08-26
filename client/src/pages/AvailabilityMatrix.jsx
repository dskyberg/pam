import { useState } from "react";
import { useQuery } from "@apollo/client";
import { GET_FULL_MATRIX, GET_AVAILABILITY_FOR } from "../graph";
import { styled } from "@mui/material/styles";
import Paper from "@mui/material/Paper";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell, { tableCellClasses } from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import TableFallack from "../components/TableFallback";

import { appStore } from '../state/appStore';


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
  "&:nth-of-type(odd)": {
    backgroundColor: theme.palette.action.disabledBackground,
  },
  // hide last border
  "&:last-child td, &:last-child th": {
    border: 0,
  },
}));


function AvailabilityCell({ itemId, jurisdictionId, onClick, ...rest }) {
  const { loading, error, data } = useQuery(GET_AVAILABILITY_FOR, {
    variables: {
      itemId,
      jurisdictionId
    }
  });

  if (error) {
    console.log('AvailabilityCell error:', error.message);
  }
  if (loading || data.availabilityFor == null) return (
    <StyledTableCell onClick={() => onClick(data.availability)} {...rest} />
  )

  const a = data.availabilityFor;
  const value = [a.lifecycle.name ?? "", a.compliance.name ?? ""].join(", ");

  return (<StyledTableCell onClick={() => onClick(a)} {...rest}>
    {value}
  </StyledTableCell>)

}

const jurisdictions = (jurisdictions, item, onClick) => {

  return jurisdictions.map((j) =>
    <AvailabilityCell key={j.id + ":" + item.id} itemId={item.id} jurisdictionId={j.id} onClick={onClick} />)
};

function Matrix({ matrix, onClick }) {

  let rows = [];
  let jurisdiction_cols = matrix.jurisdictions.length;

  const handleClick = (item) => {
    appStore.send({ type: 'setActiveItem', item })
  }


  for (let category of matrix.categories) {
    for (let product of category.products) {
      rows.push(
        <StyledTableRow key={"row" + category.id + product.id + "_1"}>
          <StyledTableCell key={category.id + "_1"}> {category.name}</StyledTableCell>
          <StyledTableCell key={product.id + "_1"} onClick={() => handleClick(product)}>{product.name}</StyledTableCell>
          <StyledTableCell key={product.id + "_feature" + "_1"} onClick={() => handleClick(null)} ></StyledTableCell>
          {[...jurisdictions(matrix.jurisdictions, product, handleClick)]}
        </StyledTableRow>,
      );
      for (let feature of product.features) {
        rows.push(
          <StyledTableRow key={"row" + category.id + product.id + feature.id + "_2"}>
            <StyledTableCell key={category.id + "_2"}> {category.name}</StyledTableCell>
            <StyledTableCell key={product.id + "_2"}>{product.name}</StyledTableCell>
            <StyledTableCell key={feature.id + "_feature" + "_2"} onClick={() => handleClick(feature)}>{feature.name}</StyledTableCell>
            {[...jurisdictions(matrix.jurisdictions, feature, handleClick)]}
          </StyledTableRow>,
        );
      }
    }
  }

  return rows;
}

export default function AvailabilityMatrix() {
  const [open, setOpen] = useState(false);
  const [windowHeight, setWindowHeight] = useState(720);
  const { loading, error, data } = useQuery(GET_FULL_MATRIX);
  const [availability, setAvailability] = useState(null);
  const [item, setItem] = useState(null)

  const handleClickOpen = (item, availability) => {
    setItem(item)
    setAvailability(availability)
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };


  if (loading) {
    return <TableFallack />;
  }

  if (error) {
    return <Paper sx={{ width: "100%", overflow: "hidden" }}>{error}</Paper>;
  }


  return (
    <div sx={{ width: "100%", overflow: "hidden", height: windowHeight, marginTop: "1em" }}>
      <TableContainer sx={{ width: "100%", maxHeight: windowHeight, padding: 0 }}>
        <Table stickyHeader aria-label="sticky table">
          <TableHead>
            <TableRow>
              <StyledTableCell style={{ minWidth: 170 }}>{"Category"}</StyledTableCell>
              <StyledTableCell style={{ minWidth: 170 }}>{"Product (SKU)"}</StyledTableCell>
              <StyledTableCell style={{ minWidth: 170 }}>{"Feature"}</StyledTableCell>
              {data.matrix.jurisdictions.map((jurisdiction) => (
                <StyledTableCell key={jurisdiction.id} style={{ minWidth: 100 }}>
                  {jurisdiction.name}
                </StyledTableCell>
              ))}
            </TableRow>
          </TableHead>
          <TableBody>
            <Matrix matrix={data.matrix} onClick={handleClickOpen} />
          </TableBody>
        </Table>
      </TableContainer>
    </div>
  );
}
