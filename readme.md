This project is now canceled...


# Workspaces

## SSP 

Handles the ssp file, gives access to internal structures and files


'''
RUST_LOG=ssp,ssp_schema,ssp_test cargo run

RUST_LOG=debug=ssp,ssp_schema,ssp_test cargo run

'''

WHen loading fmu, make sure that all files inside have read permission

## SSP-Schema

Defines the de-serializing and serializing of SSP xml files to rust structures



# Git 
'''
git submodule init
git submodule update
'''



# Validate xml

catalog.xml maps namespace to file

