link ".npmrc" {
  to = "~/.npmrc"
}

link "lel.txt" {
  to = "~/lel.txt"
}

packages {
  list    = ["fzf", "git", "tig"]
  manager = ["brew"]
}
