# Illuvium land bot
This bot reads ImmutableX API and checks if there is a new listing for Illuvium land in the last minute.

## Env
The `dotenvy` crate is used to load environment variables `BOT_TOKEN` and `CHAT_ID` from `.env` file

## Build and run
The built executable is meant to be run on a scheduled interval, e.g. with `crontab`:

```
*/1 * * * * /illuvium-land-runner >> /illuvium-land-runner.log 2>&1
```

As of time of writing ImmutableX API doesn't support any kind of `Publish/Subscribe` style, thus API is queried every given time interval.