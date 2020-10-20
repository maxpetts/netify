//@ts-ignore
import "https://deno.land/x/dotenv/load.ts";
import { encode } from "https://deno.land/std@0.74.0/encoding/base64.ts";
import { Application } from "https://deno.land/x/abc@v1.0.3/mod.ts";
import urlcat from "https://deno.land/x/urlcat@v2.0.4/src/index.ts";

const redirect_uri = "http://localhost:8080/callback";
const cliID = Deno.env.get("cli_id");
const cliSecret = Deno.env.get("cli_secret");

const app = new Application();

var generateRandomString = function (length: number) {
  var text = "";
  var possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

  for (var i = 0; i < length; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
};

app.file("/", "public/index.html").start({ port: 8080 });

app.get("/login", (res) => {
  var state = generateRandomString(16);
  let scope = "user-read-private user-read-email";

  res.redirect(urlcat("https://accounts.spotify.com/authorize", {
    client_id: cliID,
    response_type: "code",
    redirect_uri: redirect_uri,
    state: state,
    scope: scope
  }));
});

app.get("/callback", async (res) => {
  // Check for permission denied - '?error=access-denied'
  //      Just check for ?error in response string

  var code: string = res.url.searchParams.get('code') || '';
  var state: string = res.url.searchParams.get('state') || '';

  console.log(`code: ${code}`);
  console.log(`state: ${state}`);

  const tokenOptions: URLSearchParams = new URLSearchParams ({
    grant_type: "authorization_code", 
    code: code, 
    redirect_uri: redirect_uri 
  });
  
  const response = await fetch("https://accounts.spotify.com/api/token", {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
      Authorization: "Basic " + encode(cliID + ":" + cliSecret),
    },
    body: tokenOptions
  });

  let result:any = await response.json();

  let access_token = result.access_token;
  let refresh_token = result.refresh_token;

  console.log(`token: ${result.access_token}`);

  const fetchDeets = await fetch("https://api.spotify.com/v1/me", {
    headers: {
      Authorization: result.token_type + ' ' + access_token
    }
  })  

  const deets:any = await fetchDeets.json();
  return 'hi ' + deets.display_name;

  // Send tokens back to browser
  res.redirect(urlcat('/', {
    access_token: access_token,
    refresh_token: refresh_token
  }));
});
