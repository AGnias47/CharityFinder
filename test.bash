#!/bin/bash
#
#   A. Gnias
#
#   Kubuntu 5.16.5
#   Linux 5.3.0-40-generic #32-Ubuntu
#   GNU bash, version 5.0.3(1)-release

curl -X GET --header "Accept: application/json" "https://api.data.charitynavigator.org/v2/Organizations?app_id=ID&app_key=KEY"
