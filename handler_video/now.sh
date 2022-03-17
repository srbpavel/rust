#!/bin/bash
# https://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO-5.html
# #NOW for ACTIX static
#*/1 * * * * /home/conan/soft/rust/handler_video/now.sh 1>/home/conan/soft/rust/handler_video/1_cron_now.log 2>/home/conan/soft/rust/handler_video/2_cron_now.log

FILE=/home/conan/soft/rust/handler_video/static/now.txt
NOW=date
echo `$NOW -R` > $FILE
echo `$NOW +%s%N` >> $FILE
echo `$NOW +%s%6N` >> $FILE
echo `$NOW +%s%3N` >> $FILE
echo `$NOW +%s` >> $FILE
