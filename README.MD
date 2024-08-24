![Alt text](doc/banner.png?raw=true "EXTRA LIFE")

Hey,

Its a small program in rust to help you having an 'extra life' ❤️ in case of your beacon/implant crash.
Extra life will monitor a process, and every time this process crash, extra life will respawn the process.

- Without arguments extralife will monitor and re-spawn it's parent process.
- But you can also specify a pid as argument to monitor an other process, example: `extralife.exe 4994`

# example with Sliver C2

Add an extra life to your running Sliver.

![Alt text](doc/add_extra-life.png?raw=true "Add an extra life")

As you see the process tree, you got one extra life

![Alt text](doc/yougot_an_extra_life.png?raw=true "Process tree of your extra life")

Demonstration, extra life give you an other session if you die:

![Alt text](doc/extra_life.png?raw=true "EXTRA LIFE")

You are the child of extralife.exe now as you see in the process tree:

![Alt text](doc/extra-life_save-you.png?raw=true "Process tree after death")

If you want to die/crash the beacon, you can remove extra life, your beacon program will continue to run but not forever:

![Alt text](doc/remove_extra_life.png?raw=true "Remove")

## compilation

I test it only for windows, you can compile it like that:

```
cargo build --release --target x86_64-pc-windows-gnu
```

## pro tips

if you execute extralife.exe twice, you will have you 2 extra life !! 💕

## improvement todo :
- remember the commandline of the target process to recreate exact same process.
- and reuse the working dir of the target process, just in case.