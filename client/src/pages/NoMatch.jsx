import { Box } from "@mui/material";
import { styled } from '@mui/material/styles';
import Typography from "@mui/material/Typography";
import WarningAmberIcon from '@mui/icons-material/WarningAmber';
import HomeIcon from '@mui/icons-material/Home';
import Link from "@mui/material/Link";

const Headline = styled(WarningAmberIcon)(({ theme }) => ({
    color: theme.palette.warning.dark,
    padding: theme.spacing(1),
    fontSize: '6rem',
    fontWeight: 900,
}));

const MyBox = styled(Box)(({ theme }) => ({
    display: "flex",
    textAlign: "center",
    alignItems: "center",
    padding: theme.spacing(1),
}));

export default function NoMatch() {
    return (
        <MyBox>
            <Headline />
            <Typography variant="h4" sx={{ mb: 2 }}>
                That page does not exist!
                <br />
                Go back or go
            </Typography>
            <Link href="/"><HomeIcon sx={{ fontSize: '4rem' }} /></Link>
        </MyBox>
    )
}