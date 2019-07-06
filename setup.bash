cmd="jsp go --shell=bash"

function jspgo { 
    FOO=$($cmd "$@") > /dev/null; 
    if [[ $FOO =~ \s*(Error|Info).* ]] ;
    then
        echo $FOO 
    else
        eval $FOO
    fi
}
export JSP_PATH=~/etc/template.jspt