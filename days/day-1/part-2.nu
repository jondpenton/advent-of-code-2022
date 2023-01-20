#!/usr/bin/env nu

open ($env.FILE_PWD | path join `input.txt`)
  | lines
  | split list ``
  | par-each { |elf|
      $elf | reduce --fold 0 { |calories_str, total|
        $total + ($calories_str | into int)
      }
    }
  | sort --reverse
  | take 3
  | math sum
