workflow "push" {
  on = "push"
  resolves = [
    "push-wincompile",
  ]
}

action "push-wincompile" {
  uses = "./.github/docker/wincompile"
}

workflow "release" {
  resolves = [
    "release-Filter published",
    "release-publish",
  ]
  on = "release"
}

action "release-wincompile" {
  uses = "./.github/docker/wincompile"
  args = "--release --out-dir out -Z unstable-options"
}

action "release-Filter published" {
  uses = "actions/bin/filter@master"
  args = "action published"
}

action "release-publish" {
  uses = "judge2020/github-action-publish-binaries@master"
  needs = ["release-wincompile"]
  args = "out/*"
  secrets = ["GITHUB_TOKEN"]
}
