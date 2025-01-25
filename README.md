# 💦 sueur-rs

Based on data from the [Free Exercise DB 💪](https://github.com/yuhonas/free-exercise-db).

## Usage

General help menu:
```console
coko7@example:~$ sueur -h
Efficiently query workout exercises

Usage: sueur [OPTIONS] <COMMAND>

Commands:
  get   Get all data relating to a specific exercise
  list  Get the list of exercises matching the filters
  help  Print this message or the help of the given subcommand(s)

Options:
      --data <DATA_PATH>  Path to the data directory (includes JSON and images)
  -v, --verbose...        Increase logging verbosity
  -q, --quiet...          Decrease logging verbosity
  -h, --help              Print help
```

Using the `list` subcommand:
```console
coko7@example:~$ sueur help ls
Get the list of exercises matching the filters

Usage: sueur list [OPTIONS]

Options:
  -f, --force <FORCE>          [possible values: pull, push, static]
  -l, --level <LEVEL>          [possible values: beginner, intermediate, expert]
  -m, --mechanic <MECHANIC>    [possible values: compound, isolation]
  -e, --equipment <EQUIPMENT>  [possible values: bands, barbell, "body only", cable, dumbbell, "exercise ball", "e-z curl bar", "foam roll", kettlebells, machine, "medicine ball", other]
  -v, --verbose...             Increase logging verbosity
  -c, --category <CATEGORY>    [possible values: cardio, "olympic weightlifting", plyometrics, powerlifting, strength, stretching, strongman]
  -q, --quiet...               Decrease logging verbosity
  -h, --help                   Print help
```

Getting a specific exercise:
```console
coko7@example:~$ sueur get Sit-Up
{"id":"Sit-Up","name":"Sit-Up","force":"pull","level":"beginner","mechanic":"isolation","equipment":"body only","primaryMuscles":["abdominals"],"secondaryMuscles":[],"instructions":["Lie down on the floor placing your feet either under something that will not move or by having a partner hold them. Your legs should be bent at the knees.","Place your hands behind your head and lock them together by clasping your fingers. This is the starting position.","Elevate your upper body so that it creates an imaginary V-shape with your thighs. Breathe out when performing this part of the exercise.","Once you feel the contraction for a second, lower your upper body back down to the starting position while inhaling.","Repeat for the recommended amount of repetitions."],"category":"strength","images":["Sit-Up/0.jpg","Sit-Up/1.jpg"]}
```
