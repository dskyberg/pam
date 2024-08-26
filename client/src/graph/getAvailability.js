import { gql } from "@apollo/client";

export const GET_AVAILABILITY = gql`query GetAvailability($id: String!) {
    availability(id: $id) {
        id
        jurisdiction {
            id
        }
        lifecycle {
            id
            name
        }
        compliance {
            id
            name
        }
        lastUpdated
        comments {
            id
            text
            created
        }
    }
}`;

