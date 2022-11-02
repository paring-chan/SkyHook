# SkyHook

Very simple Keyboard & Mouse hook, meant for rhythm games.

## Troubleshooting

### Hook does not work on Linux

You should add `input` role for the user.

```sh
sudo usermod -aG input $USER
```
