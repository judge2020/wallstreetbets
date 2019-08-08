workflow "push" {
  on = "push"
  resolves = [
    "push-linux compile",
    "push-wincompile",
  ]
}

action "push-linux compile" {
  uses = "docker://rustlang/rust:nightly"
  runs = "cargo"
  args = "build"
}

action "push-wincompile" {
  uses = "./.github/docker/wincompile"
}

workflow "New workflow" {
  on = "push"
  resolves = ["release-wincompile", "release-Filter published"]
}

action "release-wincompile" {
  uses = "./.github/docker/wincompile"
  args = "--release "
}

action "release-Filter published" {
  uses = "actions/bin/filter@master"
}
