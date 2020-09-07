SHELL=bash

.PHONY: get_data clean

get_data:
	@$(SHELL) foo.sh || printf "Error!\n"

clean:
	rm data/*.html.gz
