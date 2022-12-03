#!/bin/bash

day=""

if [[ $# != 1 ]]
then
  read -p "What day is it? " day
else
  day=$1
fi

if [[ ${#day} == 1 ]]
then
  p="0${day}"
elif [[ ${#day} == 2 ]]
then
  p="${day}"
else
  echo "Please enter a one or two digit day" && exit 1
fi

d="Day${p}"
mkdir "${d}" || exit 1
(cd .template && cargo clean)
cp -rv .template/** "${d}"
find ${d}/src -type f -exec sed -i 's/\$day\$/'${day}'/g' {} \; -exec sed -i 's/\$day_padded\$/'${p}'/g' {} \;
