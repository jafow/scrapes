SHELL=bash

.PHONY: get_data clean

output.csv: get_data
	cargo run

get_data:
	@$(SHELL) get.sh || printf "Error!\n"

clean:
	rm data/*.html.gz data/output.csv
