# Rscroll

This project is based on [Zscroll](https://github.com/noctuid/zscroll)

## Example usage

#### Simple usage

```sh
$ rscroll "A very long text that you want to apply scroll effet......."
```

### Using commands

```sh
$ rscroll -c --command-delay 1000 "./get_player_status.sh"
```

### Options

```
  --help           # Show help menu
  --delay          # Delay in milliseconds to update dext
  --length         # Max length to text display
  --separator      # A separator character placed between texts
  --check-command  # Tell if the text is a command to be run
  --command-delay  # Delay in milliseconds to rerun the command
```

## Why?

For fun and learning purposes, but also for myself
