import { gql } from "@apollo/client";

export const GET_PRODUCT = gql`query GetProduct($name: String, $id: String) {
    product(name: $name, id: $id) {
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
        features {
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
    }
}`;

