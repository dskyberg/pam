import { gql } from "@apollo/client";

export const GET_FEATURE = gql`query GetFeature($name: String, $id: String) {
    feature(name: $name, id: $id) {
        id
        name
        availability {
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
        }
    }
}`;

