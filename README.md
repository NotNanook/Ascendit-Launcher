<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="img/Logo.png">
    <img alt="Logo" src="img/LogoDark.png" width=500>
  </picture>

  <h3 align="center">Ascendit Launcher</h3>
  
  <p align="center">A lauchner made in Tauri for all current Ascendit-based Clients</p>
  <br>
  
  <a href="https://github.com/NotNanook/Ascendit-Launcher">
    <img src="img/Launcher.jpg" alt="Logo" width="500">
  </a>
</div>


# Features
- Automatically executes and injects cheat into a supported game
- Modify default path and auto-execute in `config.json`
- Auto Update
- Small binary size (fuck electron)

# Building
- Make sure `npm` is installed -> run `npm run tauri build`
- Use the nightly build for rust

# Todo
- [X] Add auto update
- [X] Add LoadLibrary injector
- [ ] Add custom config support
- [ ] Add custom injection methods
