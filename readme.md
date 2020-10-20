# Netify 🎧
An improved spotify client, written in Deno 🦖.

## Problem Statement

The mobile app for Spotify is lackluster in indexing songs by genre, bpm and similiar songs. Losing your old discover weekly is always a pain; along with that one song on a random playlist you listened to 2 weeks ago.

## Aim

So the aim of this project is to provide all the functions that I feel are lacking in the Spotify mobile app. Will also give me some opportunity to see what you freaks listen to - hopefully.

## Todo:

- [x] Send a GET request to the authorize end point.
- [x] Hide access code secrets 😱.
- [x] Exchange authorization code for access code.
- [ ] Handle authorization errors if user denies access.
- [ ] Use access code to get user details.
- [ ] Create database to store details.
- [ ] Create auto requesting refresh token upon token expiry.
- [ ] Decide on authorization [scopes](https://developer.spotify.com/documentation/general/guides/scopes/) - *may be benefitial to do this as functionality develops* 
- [ ] List the users music library.
- [ ] Add filter functionality to the library.
- [ ] Download Discover Weekly, weekly.