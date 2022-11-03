# SkyHook

Very simple Keyboard & Mouse hook, meant for rhythm games.

## Troubleshooting

### Hook does not work on Linux

You should add `input` role for the user.

```sh
sudo usermod -aG input $USER
```

and re-login

## Credits

- Windows API from [winsafe](https://github.com/rodrigocfd/winsafe)
- MacOS listener from [rdev](https://github.com/Narsil/rdev)
- Linux event reader from [StackOverFlow](https://stackoverflow.com/a/69745021)
