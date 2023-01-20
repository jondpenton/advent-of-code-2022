#!/usr/bin/env nu

open ($env.FILE_PWD | path join `input.txt`)
  | lines
  | split list ``
  | reduce --fold 0 { |elf, max_calories|
      let elf_calories = (
        $elf | reduce --fold 0 { |calories_str, total|
          $total + ($calories_str | into int)
        }
      )

      [$max_calories, $elf_calories] | math max
    }
