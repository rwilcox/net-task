# Managing Releases

Everything is managed from Cargo.toml. Increase the version number - (patch, minor, major) and when you are ready run `net-task -t ./net-task-ci.yml run ci:gh:create-release-draft`

This will create a Release in the draft state. Artifacts will be uploaded to the same.

From there just run CI.
