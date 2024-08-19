import * as jurisdiction from "./jurisdiction.js";
import * as compliance from "./compliance.js";
import * as lifecycle from "./lifecycle.js";
import * as cell from "./cell.js";
import * as availability from "./availability.js";

async function main() {
  try {
    await lifecycle.add();
    console.log("Lifecycles added");
    await compliance.add();
    console.log("Compliances added");
    await jurisdiction.add();
    console.log("Jurisdictins added");
    await cell.add();
    console.log("Cells added");
    await availability.add();
    console.log("Categories, products, and features added");
  } catch (err) {
    console.log(err);
  }
}

main();
