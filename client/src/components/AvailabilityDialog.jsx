import { useState } from "react";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import Paper from "@mui/material/Paper";
import Draggable from "react-draggable";
import { FormControl, InputLabel, Select, MenuItem } from "@mui/material";

function PaperComponent(props) {
  return (
    <Draggable handle="#draggable-dialog-title" cancel={'[class*="MuiDialogContent-root"]'}>
      <Paper {...props} />
    </Draggable>
  );
}


export default function AvailabilityDialog({ open, onClose, item, availability }) {
  if (item == null) return null;

  const [lifecycle, setLifecycle] = useState(availability.lifecycle.name)

  const [compliance, setCompliance] = useState(availability.compliance.name)

  const handleLifecycleChange = (event) => {
    setLifecycle(event.target.value);
  };

  const handleComplianceChange = (event) => {
    setCompliance(event.target.value);
  };

  return (

    <Dialog open={open} onClose={onClose} PaperComponent={PaperComponent} aria-labelledby="draggable-dialog-title">
      <DialogTitle style={{ cursor: "move" }} id="draggable-dialog-title">
        {item}
      </DialogTitle>
      <DialogContent>

        <FormControl variant="standard" fullWidth sx={{ margin: '1em' }}>
          <InputLabel id="lifecycle-label">Lifecycle</InputLabel>
          <Select
            labelId="lifecycle-label"
            id="lifecycle"
            value={lifecycle}
            label="Lifecycle"
            onChange={handleLifecycleChange}
          >
            <MenuItem value={'BETA'}>Beta</MenuItem>
            <MenuItem value={'LEA'}>Limited Early Access</MenuItem>
            <MenuItem value={'EA'}>Early Access</MenuItem>
            <MenuItem value={'GA'}>Generally Available</MenuItem>
            <MenuItem value={'EOL'}>End Of Life</MenuItem>
          </Select>
        </FormControl>

        <FormControl variant="standard" fullWidth sx={{ margin: '1em' }}>
          <InputLabel id="compliance-label">Compliance</InputLabel>
          <Select
            labelId="compliance-label"
            id="compliance"
            value={compliance}
            label="Compliance"
            onChange={handleComplianceChange}
          >
            <MenuItem value={'Not Available'}>Not Available</MenuItem>
            <MenuItem value={'Available'}>Available</MenuItem>
            <MenuItem value={'Audit Ready'}>Audit Ready</MenuItem>
            <MenuItem value={'Authorized'}>Authorized</MenuItem>
          </Select>
        </FormControl>

        <DialogContentText>
          Last Updated: {availability.lastUpdated}
        </DialogContentText>
      </DialogContent>
      <DialogActions>
        <Button autoFocus onClick={onClose}>
          Close
        </Button>
      </DialogActions>
    </Dialog>
  );
}
