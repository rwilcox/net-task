# The net-task.yml spec

Each taskfile MUST have:

  * `version` <-- currently only 1
  * `tasks` <-- a map of names and their task information
  * `externals` <-- an optional list of URLs where additional taskfiles live

A task definition MUST include:
  * `command`: what to execute
  * `script`: given as stdin to the command

It is recommended to also have `os`

## example taskfile

```yaml

version: 1
externals:
  - ./net-task-base.yml
tasks:
  doit:
    os: any
    command: python3
    script: |
      print("hello world")


```
