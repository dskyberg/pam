import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import { ApolloClient, InMemoryCache, ApolloProvider } from "@apollo/client";

import { ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import App from "./App";
import theme from "./theme";

const client = new ApolloClient({
  uri: "http://localhost:8080/graph/graphql",
  cache: new InMemoryCache(),
});

const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(
  <ThemeProvider theme={theme}>
    <CssBaseline />
    <BrowserRouter>
      <ApolloProvider client={client}>
        <App />
      </ApolloProvider>
    </BrowserRouter>
  </ThemeProvider>,
);
