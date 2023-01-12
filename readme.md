# Netify 🎧

An improved spotify client, written in Deno 🦖.

[![wakatime](https://wakatime.com/badge/github/maxpetts/netify.svg)](https://wakatime.com/badge/github/maxpetts/netify)

## Problem Statement

The mobile app for Spotify is lackluster in indexing songs by genre, bpm and similiar songs. Losing your old discover weekly is always a pain; along with that one song on a random playlist you listened to 2 weeks ago.

## Aim

To enforce the social area of music that is missing in modern mobile based spotify. Will also give me some opportunity to see what you freaks listen to - hopefully.

# Running

App : Navigate into `yewapp` and run `trunk serve`. You may need to install trunk first with: `cargo install trunk`.

The deno webserver will most likely be deprecated.

## Todo:

- [x] Send a GET request to the authorize end point.
- [x] Hide access code secrets 😱.
- [x] Exchange authorization code for access code.
- [ ] Handle authorization errors if user denies access.
- [ ] Use access code to get user details.
- [ ] Create database to store details.
- [ ] Create auto requesting refresh token upon token expiry.
- [ ] Decide on authorization [scopes](https://developer.spotify.com/documentation/general/guides/scopes/) - _may be benefitial to do this as functionality develops_
- [ ] List the users music library.
- [ ] Add filter functionality to the library.
- [ ] Download Discover Weekly, weekly.
- [ ] Seperate hash for FE -> BE & BE -> Spotify
- [ ] trpc?
- [ ] Move token exp to BE
