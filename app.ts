//@ts-ignore
import { Application } from 'https://deno.land/x/abc@v1.0.3/mod.ts';
import urlcat from 'https://deno.land/x/urlcat@v2.0.4/src/index.ts';
import "https://deno.land/x/dotenv/load.ts";
import { encode } from "https://deno.land/std@0.74.0/encoding/base64.ts";

const redirect_uri = 'http://localhost:8080/callback';

const app = new Application();

app
  .file("/", "public/index.html")
  .start({ port: 8080 })

app.get("/login", (res) => {
  res.redirect(urlcat('https://accounts.spotify.com/authorize', {
    client_id: Deno.env.get('cli_id'),
    response_type: 'code',
    redirect_uri: redirect_uri
  }));
})

app.get("/callback", async (res) => {

  // Check for permission denied - '?error=access-denied'
  //      Just check for ?error in response string

  // Split response at '?' and strip code= or state=
  var code = res.request.url.split('code=')[1] || null; 
  // var state = res.request.state || null;

  let options = {
    grant_type: 'authorization_code',
    code: code,
    redirect_uri: redirect_uri
  };

  const response = await fetch('https://accounts.spotify.com/api/token', {
    method: 'POST',
    headers: {
      'Content-Type': 'text/json',
      'Authorization': 'Basic' + encode(Deno.env.get('cli_id') + ':' + Deno.env.get('cli_secret'))
    },
    body: JSON.stringify(options),
  });
console.log(response);
console.log('headers: ' + response.headers);
  const data = await response.headers.get('access_token');
  console.log('data: ' + data);
  // console.log(response.toString);

  const name = await fetch('https://api.spotify.com/v1/me', {
    headers: {
      'Authorization': 'Bearer ' + 'token'
    }
  })

})