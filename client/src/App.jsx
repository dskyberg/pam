import { useEffect } from "react";
import { Routes, Route } from "react-router-dom";
import { useSelector } from "@xstate/react";
import { Box } from '@mui/material';


import MenuAppBar from './components/MenuAppBar';
import ClippedDrawer from './components/ClippedDrawer';
import PanelDrawer from "./components/PanelDrawer";

import Layout from './pages/Layout';
import Home from './pages/Home';
import AvailabilityMatrix from './pages/AvailabilityMatrix';
import Jurisdictions from "./pages/Jurisdictions";
import Cells from './pages/Cells';
import About from './pages/About';
import NoMatch from './pages/NoMatch';



function App() {

  return (
    <Box sx={{ display: 'flex', width: '100%', height: "100%" }}>
      <MenuAppBar />
      <ClippedDrawer />
      <Routes>
        <Route path="/" element={<Layout />} >
          <Route index element={<Home />} />
          <Route path="matrix" element={<AvailabilityMatrix />} />
          <Route path="jurisdictions" element={<Jurisdictions />} />
          <Route path="cells" element={<Cells />} />
          <Route path="about" element={<About />} />
          <Route path="*" element={<NoMatch />} />
        </Route>
      </Routes>
      <PanelDrawer />
    </Box>
  );
}

export default App;
