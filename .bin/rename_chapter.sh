#!/bin/bash

old_title = ${1}

new_title = ${2}

rm "../${old_title}/${old_title}.tex"
mv "./TeX_files/${old_title}.tex ./TeX_files/${new_title}.tex"
mv "../${old_title}" "../${new_title}"
ln -s "./TeX_files/${new_title}.tex" "../${new_title}/${new_title}.tex" 

echo new_title
