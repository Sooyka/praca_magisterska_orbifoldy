#!/bin/bash

chapter_title=${1}

mkdir "../${chapter_title}"
touch "../${chapter_title}/${chapter_title}.tex"
echo "\\chapter{${chapter_title}}" > "../${chapter_title}/${chapter_title}.tex"
ln -s "../${chapter_title}/${chapter_title}.tex" "../TeX_files/${chapter_title}.tex"
echo "\\input{\"../${chapter_title}/${chapter_title}.tex\"}"


