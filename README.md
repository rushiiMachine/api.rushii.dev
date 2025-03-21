# api.rushii.dev

Various little services I run for internal purposes.
Currently, this handles the following:

- Aliucord Contributors API for [Aliucord Manager](https://github.com/Aliucord/Manager)
- [WIP] Badges management (Github Sponsors perks)

## Environment Variables

| PORT           | TYPE   | Default               | Description                                                   |
|----------------|--------|-----------------------|---------------------------------------------------------------|
| `PORT`         | u16    | crash or 8000 (debug) | The port the server should listen on                          |
| `GITHUB_TOKEN` | String | crash                 | THe GitHub API token used for fetching Aliucord contributors. |
