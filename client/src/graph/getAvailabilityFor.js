import { gql } from "@apollo/client";

export const GET_AVAILABILITY_FOR = gql`query GetAvailability($itemId: String!, $jurisdictionId: String!) {
    availabilityFor(itemId: $itemId, jurisdictionId: $jurisdictionId) {
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

