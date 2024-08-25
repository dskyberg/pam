import { gql } from "@apollo/client";

export const GET_CELLS = gql`query GetCells {
    cells {
        id
        name
        csp
        country
        region
        cspRegion
  }
}`;

