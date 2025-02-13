#!/bin/sh
# Create a new project from lab_template
# Usage: new_project.sh project_name
echo
template_project="lab_template"
new_project="$1"
echo "creating new project ${new_project} from ${template_project}"
if [ -z "$new_project" ]; then
    echo "ERROR: missing parameter"
    echo "Usage: new_project.sh project_name"
    exit 2
fi
if [ ! -d "${template_project}" ] ; then
    echo "ERROR: template project directory missing or not a directory: ${template_project}"
    echo "Usage: this script must be run from the repositry root directory"
    echo "Usage: new_project.sh project_name"
    exit 1
fi
#
printf "copying template directory ... "
if [ ! -d "${new_project}" ] ; then
  cp -a "${template_project}" "${new_project}" 
  if [ "$?" != "0" ]; then
    printf "[error]\nERROR: copy failed\n"
    exit 1
  fi
  printf "[done]\n"
else
  printf "[skipped]\n - new project directory already exists: ${new_project}\n"
fi
#
printf "entering new project directory ... "
cd "${new_project}" 
if [ "$?" != "0" ]; then
    printf "[error]\nERROR: cd ${new_project} failed\n"
    exit 1
fi
printf "[done]\n"
#
printf "search and replace template name by new project name:\n"
tname=$(echo ${template_project} | tr '_' ' ')
nname=$(echo ${new_project} | tr '_' ' ')
printf "[%s] -> [%s]\n" "${template_project}" "${new_project}" 
printf "[%s] -> [%s]\n" "${tname}" "${nname}" 
for fn in "Cargo.toml" "Makefile" "README.md" ; do
    printf "  - processing %s ... " "${fn}"
    sed -i -e 's/'"${template_project}"'/'"${new_project}"'/g' "${fn}"
    sed -i -e 's/'"${tname}"'/'"${nname}"'/g' "${fn}"
    if [ "$?" != "0" ]; then
        printf "[error]\nERROR: sed failed\n"
        exit 1
    fi
    printf "[done]\n"
done
printf "[%s] -> [%s]\n" "${template_project}::" "${new_project}::" 
for fn in src/*.rs tests/*.rs ; do
    printf "  - processing %s ... " "${fn}"
    sed -i -e 's/'"${template_project}::"'/'"${new_project}::"'/g' "${fn}"
    if [ "$?" != "0" ]; then
        printf "[error]\nERROR: sed failed\n"
        exit 1
    fi
    printf "[done]\n"
done
echo
