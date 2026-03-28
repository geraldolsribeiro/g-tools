#/usr/bin/env bash

# Installation
# mkdir -p ~/.local/share/bash-completion/completions
# cp _g_completions.sh ~/.local/share/bash-completion/completions/G

_g_completions() {
  local cur prev

  cur=${COMP_WORDS[COMP_CWORD]}
  prev=${COMP_WORDS[COMP_CWORD-1]}

  case ${COMP_CWORD} in
  1)
    COMPREPLY=($(compgen -W "help xournal" -- ${cur}))
    ;;
  2)
    case ${prev} in
    xournal)
      COMPREPLY=($(compgen -W "open" -- ${cur}))
      ;;
    # other)
    #   COMPREPLY=($(compgen -W "some other args" -- ${cur}))
    #   ;;
    esac
    ;;
  *)
    COMPREPLY=()
    ;;
  esac
}

complete -F _g_completions G
