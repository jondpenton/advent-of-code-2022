#!/usr/bin/env nu

open ($env.FILE_PWD | path join `input-instructions.txt`)
  | lines
  | par-each { |it|
      let letters = ($it | split row ` `)

      $letters | to json
    }
