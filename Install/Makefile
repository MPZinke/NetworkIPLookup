
########################################################################################################################
#                                                                                                                      #
#   created by: MPZinke                                                                                                #
#   on 2022.05.17                                                                                                      #
#                                                                                                                      #
#   DESCRIPTION:                                                                                                       #
#   BUGS:                                                                                                              #
#   FUTURE:                                                                                                            #
#                                                                                                                      #
########################################################################################################################


all:
	sudo apt-get install rust postgres

	sudo createdb NetworkIPLookup
	sudo psql -d NetworkIPLookup -f DB/Schema.sql
	sudo psql -d NetworkIPLookup -f DB/Data.sql

	bash ExportEnvironmentVariables.sh
	cd ../Source
	cargo build

	sudo cp target/debug/NetworkIPLookup /usr/local/bin/
	sudo cp ../Install/NetworkIPLookup.service /etc/systemd/system/
	sudo systemctl enable NetworkIPLookup.service
	sudo systemctl start NetworkIPLookup.service
