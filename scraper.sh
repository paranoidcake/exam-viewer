#!/bin/bash

pages=($(curl https://theleavingcert.com/exam-papers/ | grep "<a href=\"https://theleavingcert.com/exam-papers" | grep "^<li><a href=\""))

declare -a links

for page in ${pages[@]}; do
	startTrim=${page#*href=\"}
	endTrim=${startTrim/\">*/}
	if [[ $endTrim == http* ]]; then
		links+=("$endTrim")
	fi
done

for link in ${links[@]}; do
	mkdir examPages 2> /dev/null
	linkName=${link#*/exam-papers/}
	linkName=${linkName%/}
	echo $linkName
	curl $link > "examPages/$linkName"
done
