# SkyHook

Very simple Keyboard & Mouse hook, meant for rhythm games.

## Troubleshooting

### Hook does not work on Linux

You should add `input` role for the user.

```sh
sudo usermod -aG input $USER
```

and re-login.

## Credits

- Windows API from [winsafe](https://github.com/rodrigocfd/winsafe)
- Linux event reader from [StackOverFlow](https://stackoverflow.com/a/69745021)
- XInput handler from [x11-rs](https://github.com/AltF02/x11-rs/blob/master/x11/examples/xrecord.rs)
