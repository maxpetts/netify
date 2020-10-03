import { Application } from "https://deno.land/x/abc@v1.0.3/mod.ts";
import urlcat from 'https://deno.land/x/urlcat@v2.0.4/src/index.ts';

const cli_id = 'dcd5f7be4a1f450a8c23297b83a09cd3';
const cli_secret = 'd3d5eb4d4c9a4615b44d107c9cb83918';
const redirect_uri = 'http://localhost:8080/callback';

const app = new Application();

app
  .file("/", "public/index.html")
  .start({ port: 8080 })

app.get("/login", (c) => {
  c.redirect(urlcat('https://accounts.spotify.com/authorize', {
    client_id: cli_id,
    response_type: 'code',
    redirect_uri: redirect_uri
  }));
})

app.get("/callback", (c) => {
    return 'wagwan mi gi';
})