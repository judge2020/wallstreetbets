workflow "push" {
  on = "push"
  resolves = ["push-linux compile", "\t./.github/docker/wincompile"]
}

action "push-linux compile" {
  uses = "docker://rustlang/rust:nightly"
  runs = "cargo"
  args = "build"
}

action "./.github/docker/wincompile" {
  uses = "./.github/docker/wincompile"
}
