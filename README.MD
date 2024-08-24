![Alt text](doc/banner.png?raw=true "EXTRA LIFE")

Hey,

Its a small program in rust to help you having an 'extra life' ❤️ in case of your beacon/implant crash.

Without arguments extralife will re-spawn it's parent process, but you can also specify a pid as argument to monitor an other process : `extralife.exe 4994`

# example with Sliver C2

Add an extra life to your running Sliver.

![Alt text](doc/add_extra-life.png?raw=true "Add an extra life")

As you see the process tree, you got one extra life

![Alt text](doc/yougot_an_extra_life.png?raw=true "Process tree of your extra life")

Demonstration, extra life give you an other exec if you die:

![Alt text](doc/extra_life.png?raw=true "EXTRA LIFE")

The process tree is like that now: (you are the child of extralife.exe)

![Alt text](doc/extra-life_save-you.png?raw=true "Process tree after death")

If you want to die, you can remove extra life, your beacon program will continue to run:

![Alt text](doc/remove_extra_life.png?raw=true "Remove")

## pro tips

if you execute extralife.exe twice, you will got 2 extra life !! 💕

## improvement todo :
- remember the commandline of the target process to recreate exact same process.
- and reuse the working dir of the target process, just in case.