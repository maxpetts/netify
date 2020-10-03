import { Application } from 'https://deno.land/x/abc@v1.0.3/mod.ts';
import urlcat from 'https://deno.land/x/urlcat@v2.0.4/src/index.ts';
import "https://deno.land/x/dotenv/load.ts";

const redirect_uri = 'http://localhost:8080/callback';

const app = new Application();

app
  .file("/", "public/index.html")
  .start({ port: 8080 })

app.get("/login", (c) => {
  c.redirect(urlcat('https://accounts.spotify.com/authorize', {
    client_id: Deno.env.get('cli_id'),
    response_type: 'code',
    redirect_uri: redirect_uri
  }));
})

app.get("/callback", (c) => {
    return 'wagwan mi gi';
})