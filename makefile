## Alias for setupDay (used to call `make DAY=X`)
default: setupDay

# Removes leading zero from given day
SHORT_DAY := $(shell echo ${DAY} | awk 'sub(/^0*/, "", $$1)')
LONG_DAY := $(shell printf "%02d" ${DAY})
COOKIE_FILE := cookies.txt
SESSION ?= ${shell cat ${COOKIE_FILE}}
YEAR ?= ${shell date +"%Y"}

# Formatting
H=$(shell tput -Txterm setaf 3; tput bold)
B=$(shell tput bold; tput smul)
X=$(shell tput sgr0)

## Downloads necessary files and clones the template file (e.g. make DAY=02)
setupDay: solutionFiles download

## Call to run your code
run:
	cargo run $(SHORT_DAY)

## Create the solution files for the day
solutionFiles:
	@echo "${H}=== Copying template for day ${SHORT_DAY} ===${X}"
	@mkdir -p src/day${LONG_DAY}
	@cp -r template/ src/day${LONG_DAY}/
	@-sed -i '' -e "s/!DAY!/${LONG_DAY}/g" src/day${LONG_DAY}/*.*
	@-sed -i '' -e "s/!DAY!/${LONG_DAY}/g" src/day${LONG_DAY}/**/*.*
	@sed -i '' -e "s|\/\/ PATH_MARKER|#[path = \"day${LONG_DAY}\/solution.rs\"] mod day${LONG_DAY};\n\/\/ PATH_MARKER|g" src/main.rs
	@sed -i '' -e "s|\/\/ CASE_MARKER|        ${DAY} => (day${LONG_DAY}::part1, day${LONG_DAY}::part2),\n\/\/ CASE_MARKER|g" src/main.rs
	@echo "#[path = \"day${LONG_DAY}/solution.rs\"] pub mod day${LONG_DAY};" >> src/lib.rs

## Downloads the instructions and inputs for a day
download: src/day${LONG_DAY}/README.md src/day${LONG_DAY}/input.txt

src/day${LONG_DAY}/input.txt:
	@echo "${H}=== Downloading input for day ${SHORT_DAY} ===${X}"
	@curl -s -b "session=${SESSION}" https://adventofcode.com/${YEAR}/day/${SHORT_DAY}/input > src/day${LONG_DAY}/input.txt

src/day${LONG_DAY}/README.md: src/day${LONG_DAY}/challenge.html
	@echo "${H}=== Parsing input ===${X}"
	@./scripts/parse_challenge.sh ${LONG_DAY}

src/day${LONG_DAY}/challenge.html:
	@echo "${H}=== Downloading challenge for day ${SHORT_DAY} ===${X}"
	@curl -s -b "session=${SESSION}" https://adventofcode.com/${YEAR}/day/${SHORT_DAY} > src/day${LONG_DAY}/challenge.html


## Update the readme with the latest AoC stats
stats:
	@echo "${H}=== Creating Stats Table ===${X}"
	@$(eval TABLE = $(shell python3 scripts/generate_stats.py ${COOKIE_FILE} ${YEAR}))
	@sed 's/STATS_TABLE/${TABLE}/g' README_template.md | awk '{gsub(/~~/,"\n")}1' > README.md

## Create necessary files for the new repo
setup:
	@echo "${H}=== Creating Necessary Directories ===${X}"
	@mkdir -p template
	@pip3 -r scripts/requirements.txt
	@echo "${H}=== Create a template file and adjust the indicated recipe ===${X}"

## Call `make cookie SESSION=${}` to set the cookie used to download your input text
cookie:
	@echo ${SESSION} > ${COOKIE_FILE}

## Print help for the make forumlae
help:
	@sh scripts/help.sh makefile
