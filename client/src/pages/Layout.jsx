import { Outlet } from "react-router-dom";

import Container from "@mui/material/Container";
import Toolbar from "@mui/material/Toolbar";

export default function Layout() {
    return (
        <Container >
            <Toolbar />
            <Outlet />
        </Container>
    )
}