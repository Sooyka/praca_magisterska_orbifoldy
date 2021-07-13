#!/bin/bash

chapter_title=${1}

mkdir "../${chapter_title}"
touch "./TeX_files/${chapter_title}.tex"
echo -e "% mainfile: ../praca_magisterska_orbifoldy.tex\n\\chapter{${chapter_title}}" > "./TeX_files/${chapter_title}.tex"
ln -s "./TeX_files/${chapter_title}.tex" "../${chapter_title}/${chapter_title}.tex"
echo "\\input{\"./TeX_files/${chapter_title}.tex\"}"


