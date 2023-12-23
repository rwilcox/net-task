# Why Net-Task

Some build systems couple building software with executing non build related tasks in software development.

For example, most projects have a task to fix linter errors. I would argue that this isn't part of the build process (I don't think I want a robot automatically doing this on build), but a utility script provided for the developers.

Additionally, platform engineering teams may provide [developer starter packs](https://www.cnpatterns.org/development-design/developer-starter-pack) for development teams to use. These help teams boot up projects quickly, while laying down some organization-wide opinions about services.

But, the problem is: if we have a ton of services using these starter packs, do we have to duplicate the non-build related tasks in each repo? Does every repo _really_ need its own implementation of the "lint:fix" script? Or (for those organizations that enforce code coverage standards at a platform level) the "coverage:test:meets_minimum" command?

Wouldn't it be great to have a task runner that could inherit tasks from task files that live _elsewhere_? (later versions of this utility may deal with the safety aspect of downloading and running nonsense straight from the internet...)

Likewise, what if the standard "lint:fix" script doesn't work for your team? You may want to override standard task implementations. You should be able to!

## TL; DR

  * How can you DRY up all those development related tasks you have in your microservice herd? (Answer: Net-task!)
  * How can you let your teams override organization standard tasks? (while maybe forcing the standard version to run)? (Answer: Net-task!)
  * How can you do all this in the lightest way possible, across languages? (A: net-task!)

## Alternatives

Alternatives that are cross-platform, language indepedent, can be easily installed, and doesn't conflate building artifacts _neccisarily_ with running tasks. (Sorry, [Gradle](https://gradle.org/))

### Cargo-Make

[cargo-make](https://github.com/sagiegurari/cargo-make) is neat! Auto convert shell scripts neat; basic extending and modification, and complex. Written in Rust too, and can integrate with `cargo`, or work standalone

### Just

[just](https://github.com/casey/just) more closely echos the semantics of Make. Even includes built in functions, and allows inclusion of additional files.

### Task

[task](https://taskfile.dev/) is _extremely_ similar to net-task (including the YAML and the remote file path), and you should probably use that, honestly, if you're looking for simplicity.

I didn't actually know about Task when I started work on this. But net-task is my learning Rust project, so, _nerds_.


# Use Cases

  * in a multiple repo situation, put a base taskfile on (some https accessible endpoint) and point multiple repos at it, to propegate tools across your microservice herd
  * in a monorepo situation: put a base taskfile high in the directory structure and have services specify relative file paths up to it

# The net-task.yml spec

Each taskfile MUST have:

  * `version` <-- currently only 1
  * `tasks` <-- a map of names and their task information
  * `externals` <-- an optional list of URLs where additional taskfiles live

A task definition MUST include:
  * `command`: what to execute
  * `script`: given as stdin to the command

It is recommended to also have `os` and a `description`

## example taskfile

```yaml

version: 1
externals:
  - ./net-task-base.yml
  - https://gist.githubusercontent.com/rwilcox/072cfc11ca43582940bcd9caa4e8e3e0/raw/a8cf8f683b9e9aec77fda40a17e86dbaaef299cd/net-task.yml
name: local_tasks
tasks:
  doit:
    description: say hello
    os: any
    command: python3
    script: |
      print("hello world")


```
