workflow "push" {
  on = "push"
  resolves = ["push-linux compile", "\t./.github/docker/wincompile"]
}

action "push-linux compile" {
  uses = "docker://rustlang/rust:nightly"
  runs = "cargo"
  args = "build"
}

action "\t./.github/docker/wincompile" {
  uses = "\t./.github/docker/wincompile"
}
