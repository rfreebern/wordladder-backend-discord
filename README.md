# WordLadder Multiplayer

A Fastly Compute@Edge application that adds real-time multiplayer functionality to WordLadder when running inside Discord.

## Setup

1. Install dependencies:
   ```bash
   cargo build
   ```

2. Set up environment variables:
   ```bash
   export DISCORD_PUBLIC_KEY=your_discord_public_key_here
   export ORIGIN=https://discord.wordladder.fun
   ```

3. Deploy:
   ```bash
   fastly compute build
   fastly compute deploy --env prod
   ```

## Discord Developer Portal Configuration

- Activities → iframe URL: `https://wld-cdn.edgecompute.app/`
- Enable "Server Activity" scope

## Features

- **Zero-touch integration**: Works with existing WordLadder client
- **Guild-local presence**: Users see only their guild members
- **Persistent ladders**: Progress saved per user per guild
- **JWT verification**: Prevents cross-guild tampering
- **Rate limiting**: 10 writes per minute per user
- **Real-time updates**: WebSocket presence updates

## Testing

- Direct browser visit → pure single-player, no multiplayer bar
- Inside Discord guild A → sees only guild A users
- Expired JWT → WS closed 1008, PUT rejected 403
- Rate-limit → 429 after 10 rapid saves
