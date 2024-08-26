import { useQuery, useMutation } from '@apollo/client';
import { GET_FULL_MATRIX, GET_AVAILABILITY, UPDATE_AVAILABILITY } from '../graph';

import { Typography, Stack } from "@mui/material";
import { FormControl, InputLabel, Select, MenuItem } from "@mui/material";
import Comments from "./Comments";
import Loading from './Loading';


export function AvailabilityPanel({ item, matrix }) {
    const { data, loading, error } = useQuery(GET_AVAILABILITY, { variables: { id: item.id } })

    const [updateAvailability, updateResult] = useMutation(UPDATE_AVAILABILITY,
        {
            refetchQueries: [
                GET_FULL_MATRIX, // DocumentNode object parsed with gql
                GET_AVAILABILITY // Query name
            ]
        },
    );


    const handleLifecycleChange = (event) => {
        updateAvailability({
            variables: {
                input: {
                    id: item.id,
                    lifecycle: event.target.value
                }
            }
        }).catch((err) => console.log("Got an Apollo Error:", err.message))
    };

    const handleComplianceChange = (event) => {
        updateAvailability({
            variables: {
                input: {
                    id: item.id,
                    compliance: event.target.value
                }
            }
        }).catch((err) => console.log("Got an Apollo Error:", err.message))
    };

    if (loading) return <Loading />

    if (error) {
        console.log("Availability error:", error.message);
        return null
    }

    const availability = data.availability

    const Jurisdiction = () => {
        const jurisdiction = matrix.jurisdictions.find(j => j.id == availability.jurisdiction.id);
        return (
            <Typography>Jurisdiction: {jurisdiction.name}</Typography>
        )
    }
    return (
        <Stack sx={{ paddingLeft: "1em", paddingRight: "1em" }}>
            <Typography variant="h6">Availability</Typography>
            <Typography >{availability.id}</Typography>
            <Jurisdiction />
            <Typography >Updated: {availability.lastUpdated}</Typography>

            <FormControl variant="standard" fullWidth sx={{ margin: '1em' }}>
                <InputLabel id="lifecycle-label">Lifecycle</InputLabel>
                <Select
                    labelId="lifecycle-label"
                    id="lifecycle"
                    value={availability.lifecycle.name}
                    label="Lifecycle"
                    onChange={handleLifecycleChange}
                >
                    {matrix.lifecycles.map((lifecycle) => (<MenuItem key={lifecycle.id} value={lifecycle.name}>{lifecycle.description}</MenuItem>))}
                </Select>
            </FormControl>

            <FormControl variant="standard" fullWidth sx={{ margin: '1em' }}>
                <InputLabel id="compliance-label">Compliance</InputLabel>
                <Select
                    labelId="compliance-label"
                    id="compliance"
                    value={availability.compliance.name}
                    label="Compliance"
                    onChange={handleComplianceChange}
                >
                    {matrix.compliances.map(compliance => (<MenuItem key={compliance.id} value={compliance.name}>{compliance.name}</MenuItem>))}
                </Select>
            </FormControl>
            <Comments itemId={availability.id} />
        </Stack>
    );
}
