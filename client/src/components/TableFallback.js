import Skeleton from '@mui/material/Skeleton';
import Stack from '@mui/material/Stack';

export default function TableFallack() {
    return (
        <Stack spacing={1} >
            <Skeleton variant="rectangular" width={3 * 156} height={24} />
            <Stack direction="row" spacing={1}>
                <Skeleton variant="rectangular" width={150} height={24} />
                <Skeleton variant="rectangular" width={150} height={24} />
                <Skeleton variant="rectangular" width={150} height={24} />
            </Stack>
            <Stack direction="row" spacing={1}>
                <Skeleton variant="rectangular" width={150} height={24} />
                <Skeleton variant="rectangular" width={150} height={24} />
                <Skeleton variant="rectangular" width={150} height={24} />
            </Stack>
            <Stack direction="row" spacing={1}>
                <Skeleton variant="rectangular" width={150} height={24} />
                <Skeleton variant="rectangular" width={150} height={24} />
                <Skeleton variant="rectangular" width={150} height={24} />
            </Stack>
        </Stack>
    )
}
