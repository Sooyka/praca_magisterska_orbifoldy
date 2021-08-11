#!/bin/bash

touch ../.bin/${1}.sh
echo '#!/bin/bash' > ../.bin/${1}.sh

chmod u+x ../.bin/${1}.sh
ln -s ../.bin/${1}.sh ./.${1}.sh 
