// deno-lint-ignore-file
import { createServer } from "./api/Server.ts";
import { getApp } from "./WasmApp.ts";

const port = 4000;

const app = await createServer(await getApp());

app.listen(port, () => {
  console.log(`[EXPRESS APP] Server running at http://localhost:${port}`);
});
