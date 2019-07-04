#!/bin/csh

set cmdval='jsp go --shell=tcsh'
set res=`$cmdval $*`
set val = `echo $res | grep '\s*Error.*'`

if ( $val == "" ) then
    eval $res
else 
    echo $res
endif
