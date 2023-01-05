//@ts-ignore
import "https://deno.land/x/dotenv/load.ts";
import { encode } from "https://deno.land/std/encoding/base64.ts";
import { Application } from "https://deno.land/x/abc/mod.ts";
import urlcat from "https://deno.land/x/urlcat/src/index.ts";

console.debug("Running");

const redirect_uri = "http://localhost:8080/callback";
const cliID = Deno.env.get("cli_id");
const cliSecret = Deno.env.get("cli_secret");
var access_token: string;
var token_type: string;
var refresh_token: string;

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
  let url = urlcat("https://accounts.spotify.com/authorize", {
    client_id: cliID,
    response_type: "code",
    redirect_uri: redirect_uri,
    state: state,
    scope: scope
  })
  console.debug(url);
  res.redirect(url);
});

app.get("/callback", async (res) => {
  // Check for permission denied - '?error=access-denied'
  //      Just check for ?error in response string

  var code = res.url.searchParams.get('code') || '';
  var state = res.url.searchParams.get('state') || '';

  const tokenOptions: URLSearchParams = new URLSearchParams({
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

  let result: any = await response.json();

  access_token = result.access_token;
  token_type = result.token_type;
  refresh_token = result.refresh_token;

  const fetchDeets = await fetch("https://api.spotify.com/v1/me", {
    headers: {
      Authorization: token_type + ' ' + access_token
    }
  })

  const deets: any = await fetchDeets.text();

  // Send tokens back to browser
  res.redirect(urlcat('/', {
    access_token: access_token,
    refresh_token: refresh_token
  }));
});

app.get("/profile", async (res) => {
  if (access_token == "" || refresh_token == "") {
    return "error signing in"; // Network Authentication Required
  }

  return (await fetch("https://api.spotify.com/v1/me/playlists", {
    headers: {
      Authorization: token_type + ' ' + access_token
    }
  })).text();
})